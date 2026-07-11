#!/usr/bin/env python3
"""สร้างดัชนี embeddings จาก wiki/*.md → data/embeddings.json

ใช้: GEMINI_API_KEY=<key> python scripts/build-embeddings.py
ตัวเลือก:
  --wiki-dir  path ไปยัง wiki/ (default: wiki/ ใน root repo)
  --out       path ไฟล์ output (default: data/embeddings.json)
  --model     Gemini embedding model (default: models/gemini-embedding-001)
  --batch     จำนวน chunk ต่อ batch request (default: 50)

Resume: ถ้า output มีอยู่แล้ว จะข้าม chunk ที่ text ไม่เปลี่ยน (เทียบด้วย MD5 hash)
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import re
import sys
import time
import urllib.request
import urllib.error
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FRONTMATTER_RE = re.compile(r"^---\s*\n.*?\n---\s*\n", re.DOTALL)
# แบ่งตามหัวข้อระดับ 2 และ 3 (## / ###)
HEADING_RE = re.compile(r"^(#{2,3})\s+(.+)$", re.MULTILINE)

# ขนาด chunk เป้าหมาย (ตัวอักษร) — เผื่อ token limit ของ embedding model
MAX_CHARS = 1800
# overlap ระหว่าง chunk ที่ถูกแบ่งย่อย — กันเนื้อหาขาดช่วงตรงรอยต่อ
OVERLAP_CHARS = 200
# section สั้นกว่านี้ข้าม (ไม่มีสาระพอจะ embed)
MIN_CHARS = 40


def strip_frontmatter(text: str) -> tuple[str, str]:
    """คืน (title_from_frontmatter_or_h1, body_without_frontmatter)"""
    title = ""
    body = text
    m = FRONTMATTER_RE.match(text)
    if m:
        fm = m.group(0)
        body = text[m.end():]
        for line in fm.splitlines():
            if line.startswith("title:"):
                title = line.split(":", 1)[1].strip().strip('"\'')
                break
    if not title:
        for line in body.splitlines():
            if line.startswith("# "):
                title = line[2:].strip()
                break
    return title, body


def _slugify(s: str) -> str:
    return re.sub(r"[^a-zA-Z0-9ก-ฮ]", "_", s)[:40] or "x"


def split_long(text: str, max_chars: int = MAX_CHARS, overlap: int = OVERLAP_CHARS) -> list[str]:
    """แบ่งข้อความยาวเป็นหลายชิ้นตามย่อหน้า + overlap — ไม่ตัดเนื้อหาทิ้ง"""
    if len(text) <= max_chars:
        return [text]

    paras = [p.strip() for p in re.split(r"\n\s*\n", text) if p.strip()]
    pieces: list[str] = []
    cur = ""
    for p in paras:
        # ย่อหน้าเดียวยาวเกิน max → ตัดแข็งเป็นช่วงๆ (มี overlap)
        if len(p) > max_chars:
            if cur:
                pieces.append(cur)
                cur = ""
            step = max(1, max_chars - overlap)
            for i in range(0, len(p), step):
                pieces.append(p[i:i + max_chars])
            continue
        if not cur:
            cur = p
        elif len(cur) + len(p) + 2 <= max_chars:
            cur += "\n\n" + p
        else:
            pieces.append(cur)
            tail = cur[-overlap:] if overlap else ""
            cur = f"{tail}\n\n{p}" if tail else p
    if cur:
        pieces.append(cur)
    return pieces


def split_sections(body: str) -> list[tuple[str, str]]:
    """แบ่ง body ตามหัวข้อ H2/H3 → [(heading_path, section_text)]
    heading_path = 'H2' หรือ 'H2 › H3' (intro ก่อนหัวข้อแรก = path ว่าง)"""
    matches = list(HEADING_RE.finditer(body))
    sections: list[tuple[str, str]] = []

    intro_end = matches[0].start() if matches else len(body)
    intro = body[:intro_end].strip()
    if intro:
        sections.append(("", intro))

    last_h2 = ""
    for i, m in enumerate(matches):
        level = len(m.group(1))
        heading = m.group(2).strip()
        start = m.start()
        end = matches[i + 1].start() if i + 1 < len(matches) else len(body)
        text = body[start:end].strip()
        if level == 2:
            last_h2 = heading
            path = heading
        else:  # H3
            path = f"{last_h2} › {heading}" if last_h2 else heading
        sections.append((path, text))
    return sections


def chunk_file(path: Path, base: Path) -> list[dict]:
    """แบ่งไฟล์ markdown ตามหัวข้อ H2/H3 + แบ่งย่อยถ้ายาวเกิน (มี overlap)"""
    rel = path.relative_to(base).as_posix()
    file_id = path.stem
    raw = path.read_text(encoding="utf-8")
    title, body = strip_frontmatter(raw)

    chunks: list[dict] = []
    for heading_path, text in split_sections(body):
        if len(text) < MIN_CHARS:
            continue
        header = title if not heading_path else f"{title} — {heading_path}"
        slug = _slugify(heading_path) if heading_path else "intro"
        pieces = split_long(text)
        for j, piece in enumerate(pieces):
            suffix = "" if len(pieces) == 1 else f"__{j}"
            chunks.append({
                "id": f"{file_id}::{slug}{suffix}",
                "file": rel,
                "title": title,
                "heading": heading_path,
                "text": f"{header}\n\n{piece}",
            })

    return chunks


def text_hash(text: str) -> str:
    """MD5 8 หลักแรก — ใช้เป็น cache key เพื่อตรวจว่าเนื้อหาเปลี่ยนหรือไม่"""
    return hashlib.md5(text.encode()).hexdigest()[:8]


def embed_batch(texts: list[str], model: str, api_key: str) -> list[list[float]]:
    """เรียก Gemini batchEmbedContents และคืน list ของ vector"""
    url = f"https://generativelanguage.googleapis.com/v1/{model}:batchEmbedContents?key={api_key}"
    requests_body = {
        "requests": [
            {"model": model, "content": {"parts": [{"text": t}]}}
            for t in texts
        ]
    }
    body_bytes = json.dumps(requests_body).encode()
    req = urllib.request.Request(
        url, data=body_bytes,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    try:
        with urllib.request.urlopen(req, timeout=60) as resp:
            data = json.loads(resp.read())
    except urllib.error.HTTPError as e:
        err = e.read().decode(errors="replace")
        raise RuntimeError(f"Gemini HTTP {e.code}: {err[:400]}") from e

    return [item["values"] for item in data.get("embeddings", [])]


def main() -> int:
    parser = argparse.ArgumentParser(description="สร้างดัชนี embeddings จาก wiki/*.md")
    parser.add_argument("--wiki-dir", default=str(ROOT / "wiki"), help="path ไปยัง wiki/")
    parser.add_argument("--out", default=str(ROOT / "data" / "embeddings.json"), help="output JSON")
    parser.add_argument("--model", default="models/gemini-embedding-001", help="Gemini embedding model")
    parser.add_argument("--batch", type=int, default=50, help="chunk ต่อ batch")
    args = parser.parse_args()

    api_key = os.environ.get("GEMINI_API_KEY", "")
    if not api_key:
        print("ต้องตั้งค่า GEMINI_API_KEY ก่อน", file=sys.stderr)
        return 1

    wiki_dir = Path(args.wiki_dir)
    if not wiki_dir.is_dir():
        print(f"ไม่พบ wiki dir: {wiki_dir}", file=sys.stderr)
        return 1

    # เก็บ chunk ทั้งหมด
    all_chunks: list[dict] = []
    for md_file in sorted(wiki_dir.rglob("*.md")):
        try:
            chunks = chunk_file(md_file, wiki_dir)
            all_chunks.extend(chunks)
        except Exception as exc:
            print(f"ข้าม {md_file.name}: {exc}", file=sys.stderr)

    print(f"chunks ทั้งหมด: {len(all_chunks)}")

    # Resume: โหลด chunk ที่ embed แล้ว key = (id, text_hash) → ตรวจเนื้อหาเปลี่ยนด้วย
    out_path = Path(args.out)
    existing: dict[tuple[str, str], list[float]] = {}
    if out_path.exists():
        try:
            prev = json.loads(out_path.read_text(encoding="utf-8"))
            for c in prev:
                if "vector" in c:
                    h = c.get("text_hash") or text_hash(c.get("text", ""))
                    existing[(c["id"], h)] = c["vector"]
            print(f"resume: มี {len(existing)} chunks เดิมแล้ว — ข้ามไป")
        except Exception:
            pass

    for c in all_chunks:
        c["text_hash"] = text_hash(c["text"])

    pending = [c for c in all_chunks if (c["id"], c["text_hash"]) not in existing]
    print(f"ต้อง embed {len(pending)} chunks ใหม่")

    # คำนวณ delay ให้ไม่เกิน rate limit
    # free tier: 100 embed requests/min → batch ละ B items ใช้ quota B
    # delay = max(0, batch_size / rate_per_min * 61) วินาที (61 = buffer)
    RATE_PER_MIN = 90  # conservative (จริงคือ 100)
    batch_size = args.batch

    results: list[dict] = []
    for i in range(0, len(pending), batch_size):
        batch = pending[i:i + batch_size]
        texts = [c["text"] for c in batch]
        lo = i + 1
        hi = min(i + batch_size, len(pending))
        print(f"  embedding {lo}–{hi} / {len(pending)} ...", end="", flush=True)
        try:
            vectors = embed_batch(texts, args.model, api_key)
        except RuntimeError as e:
            print(f"\nข้อผิดพลาด: {e}", file=sys.stderr)
            # บันทึกที่ทำได้แล้วก่อน
            break
        for chunk, vec in zip(batch, vectors):
            existing[(chunk["id"], chunk["text_hash"])] = vec
            results.append({**chunk, "vector": vec})
        print(" ok")
        if hi < len(pending):
            delay = (batch_size / RATE_PER_MIN) * 61
            print(f"  รอ {delay:.0f}s (rate limit) ...", flush=True)
            time.sleep(delay)

    # รวมผลเดิม + ใหม่ พร้อม vector
    all_results: list[dict] = []
    for c in all_chunks:
        vec = existing.get((c["id"], c["text_hash"]))
        if vec is not None:
            all_results.append({**c, "vector": vec})

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(all_results, ensure_ascii=False, indent=None), encoding="utf-8")
    total_new = len(results)
    total_all = len(all_results)
    print(f"บันทึก {total_all} chunks ({total_new} ใหม่) -> {out_path}")
    if total_all < len(all_chunks):
        print(f"ยังขาดอีก {len(all_chunks) - total_all} chunks — รัน script อีกครั้งเพื่อ resume")
        return 2
    return 0


if __name__ == "__main__":
    sys.exit(main())
