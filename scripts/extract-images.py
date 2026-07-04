#!/usr/bin/env python3
"""ตัดรูปจาก PDF ไป assets/ ตาม workflow ย่อยความรู้."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

try:
    import fitz  # PyMuPDF
except ImportError:
    print("ติดตั้ง PyMuPDF: pip install pymupdf", file=sys.stderr)
    sys.exit(1)

ROOT = Path(__file__).resolve().parents[1]


def slugify(name: str) -> str:
    s = Path(name).stem.strip()
    s = re.sub(r"\s+", "-", s)
    return s


def extract(pdf_path: Path, out_dir: Path, min_size: int) -> list[tuple[int, str, int, int]]:
    out_dir.mkdir(parents=True, exist_ok=True)
    saved: list[tuple[int, str, int, int]] = []

    doc = fitz.open(pdf_path)
    for page_idx in range(len(doc)):
        page = doc[page_idx]
        page_no = page_idx + 1
        images = page.get_images(full=True)
        img_idx = 0
        for img in images:
            xref = img[0]
            try:
                base = doc.extract_image(xref)
            except Exception:
                continue
            w, h = base["width"], base["height"]
            if w < min_size or h < min_size:
                continue
            img_idx += 1
            ext = base["ext"]
            fname = f"p{page_no:03d}-{img_idx:02d}.{ext}"
            fpath = out_dir / fname
            fpath.write_bytes(base["image"])
            saved.append((page_no, fname, w, h))

    doc.close()
    return saved


def write_catalog(catalog_path: Path, pdf_name: str, folder: str, saved: list[tuple[int, str, int, int]]) -> None:
    lines = [
        "---",
        f'title: "Catalog — {pdf_name}"',
        "type: catalog",
        "tags:",
        "  - assets",
        "---",
        "",
        f"# Catalog — {pdf_name}",
        "",
        f"ต้นฉบับ: `raw/{pdf_name}`",
        "",
        "| หน้า | ไฟล์ | ขนาด |",
        "|------|------|------|",
    ]
    for page, fname, w, h in saved:
        lines.append(f"| {page} | ![[assets/{folder}/{fname}]] | {w}×{h} |")
    lines.append("")
    catalog_path.write_text("\n".join(lines), encoding="utf-8", newline="\n")


def main() -> int:
    parser = argparse.ArgumentParser(description="ตัดรูปจาก PDF")
    parser.add_argument("pdf", help="path ไปยัง PDF ใน raw/")
    parser.add_argument("--min-size", type=int, default=50, help="ขนาดขั้นต่ำ px (default 50)")
    parser.add_argument("--no-catalog", action="store_true", help="ไม่สร้าง catalog markdown")
    args = parser.parse_args()

    pdf_path = Path(args.pdf)
    if not pdf_path.is_absolute():
        pdf_path = ROOT / pdf_path
    if not pdf_path.exists():
        print(f"ไม่พบไฟล์: {pdf_path}", file=sys.stderr)
        return 1

    folder = slugify(pdf_path.name)
    out_dir = ROOT / "assets" / folder
    saved = extract(pdf_path, out_dir, args.min_size)

    total_bytes = sum((out_dir / f).stat().st_size for _, f, _, _ in saved)
    print(f"บันทึก {len(saved)} รูป → assets/{folder}/ ({total_bytes // 1024} KB)")

    if not args.no_catalog:
        catalog_name = f"catalog-{folder}.md"
        catalog_path = ROOT / "assets" / catalog_name
        write_catalog(catalog_path, pdf_path.name, folder, saved)
        print(f"catalog → assets/{catalog_name}")

    return 0


if __name__ == "__main__":
    sys.exit(main())
