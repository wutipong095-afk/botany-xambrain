#!/usr/bin/env python3
"""ตรวจอักษรและ typo ใน wiki / catalog markdown."""

from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]

# meta doc ที่ยกตัวอย่างคำผิด — ไม่ตรวจ/ไม่ fix
SKIP_FILES = {ROOT / "wiki" / "knowledge-extraction-pipeline.md"}

# คู่ typo ที่รู้จัก: wrong -> right
KNOWN_TYPOS: dict[str, str] = {
    "กุหลาด": "กุหลาบ",
    "พับม้วย": "พับม้วน",
    "bifoliate": "bifoliolate",
    "โcoน": "โคน",
}

# ไทย + อักษรลatin ปนในคำเดียว (ยกเว้น italic *...* และ wikilink)
MIXED_SCRIPT = re.compile(
    r"(?<![*\[/])[\u0E00-\u0E7F]+[a-zA-Z]+[\u0E00-\u0E7F]*"
    r"|[\u0E00-\u0E7F]*[a-zA-Z]+[\u0E00-\u0E7F]+(?![\]/])"
)

# รูป wikilink ที่ไม่มี assets/
BAD_IMAGE_LINK = re.compile(r"!\[\[(?!assets/)([^\]|]+/[^\]|]+)\]\]")

FRONTMATTER = re.compile(r"^---\s*\n.*?\n---\s*\n", re.DOTALL)


def strip_frontmatter(text: str) -> str:
    return FRONTMATTER.sub("", text, count=1)


def strip_code(text: str) -> str:
    text = re.sub(r"```.*?```", "", text, flags=re.DOTALL)
    return re.sub(r"`[^`]+`", "", text)


def normalize_newlines(text: str) -> str:
    return text.replace("\r\n", "\n").replace("\r", "\n")


def iter_targets(paths: list[Path]) -> list[Path]:
    if paths:
        out: list[Path] = []
        for p in paths:
            p = p if p.is_absolute() else ROOT / p
            if p.is_dir():
                out.extend(sorted(p.rglob("*.md")))
            elif p.is_file():
                out.append(p)
        return out
    wiki = ROOT / "wiki"
    catalogs = list((ROOT / "assets").glob("catalog-*.md"))
    return sorted(wiki.rglob("*.md")) + sorted(catalogs)


def check_file(path: Path) -> list[str]:
    if path.resolve() in SKIP_FILES:
        return []
    rel = path.relative_to(ROOT)
    text = path.read_text(encoding="utf-8")
    body = strip_frontmatter(text)
    prose = strip_code(body)
    issues: list[str] = []

    for wrong, right in KNOWN_TYPOS.items():
        if wrong in prose:
            issues.append(f"{rel}: typo '{wrong}' → ควรเป็น '{right}'")

    for m in MIXED_SCRIPT.finditer(prose):
        snippet = m.group(0)
        if snippet in KNOWN_TYPOS:
            continue
        issues.append(f"{rel}: อักษรปน '{snippet}'")

    for m in BAD_IMAGE_LINK.finditer(body):
        issues.append(f"{rel}: รูป wikilink ควรใช้ assets/ — ![[{m.group(1)}]]")

    return issues


def apply_fixes(path: Path) -> int:
    if path.resolve() in SKIP_FILES:
        return 0
    text = normalize_newlines(path.read_text(encoding="utf-8"))
    original = text
    for wrong, right in KNOWN_TYPOS.items():
        text = text.replace(wrong, right)
    if text != original:
        path.write_text(text, encoding="utf-8", newline="\n")
        return 1
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description="ตรวจอักษร wiki/catalog")
    parser.add_argument("paths", nargs="*", help="ไฟล์หรือโฟลเดอร์ (default: wiki/ + catalog)")
    parser.add_argument("--fix", action="store_true", help="แก้ typo จากพจนานุกรมที่รู้จัก")
    args = parser.parse_args()

    targets = iter_targets([Path(p) for p in args.paths])

    if args.fix:
        fixed = sum(apply_fixes(t) for t in targets)
        print(f"แก้ไขแล้ว {fixed} ไฟล์")
        return 0

    all_issues: list[str] = []
    for t in targets:
        all_issues.extend(check_file(t))

    if not all_issues:
        print(f"OK — ตรวจ {len(targets)} ไฟล์ ไม่พบปัญหา")
        return 0

    print(f"พบ {len(all_issues)} ปัญหา:\n")
    for issue in all_issues:
        print(issue)
    return 1


if __name__ == "__main__":
    sys.exit(main())
