#!/usr/bin/env python3
"""Merge Tier 2 menu analysis from menu-analysis-tier2-round1.json into menu-db.json."""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MENU_DB = ROOT / "data" / "menu-db.json"
ANALYSIS = ROOT / "data" / "menu-analysis-tier2-round1.json"


def main():
    db = json.loads(MENU_DB.read_text(encoding="utf-8"))
    extra = json.loads(ANALYSIS.read_text(encoding="utf-8"))

    analyses = extra.get("analyses", {})
    by_id = {m["id"]: m for m in db["menus"]}

    merged = 0
    for mid, fields in analyses.items():
        if mid not in by_id:
            print(f"skip unknown id: {mid}")
            continue
        for key, val in fields.items():
            by_id[mid][key] = val
        merged += 1

    for menu in extra.get("new_menus", []):
        mid = menu["id"]
        if mid in by_id:
            for key, val in analyses.get(mid, {}).items():
                by_id[mid][key] = val
            print(f"updated existing: {mid}")
        else:
            full = {**menu, **analyses.get(mid, {})}
            db["menus"].append(full)
            by_id[mid] = full
            print(f"added: {mid}")

    db["schema_version"] = "0.2"
    db["updated"] = extra.get("updated", db.get("updated"))
    note_extra = " · analysisTier 2 = ingredients/layerS/whenCooked (ดู wiki/food-recommender.md)"
    if note_extra not in db.get("note", ""):
        db["note"] = (db.get("note", "") + note_extra).strip()

    MENU_DB.write_text(
        json.dumps(db, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )
    print(f"merged Tier 2 into {merged} menus; total menus: {len(db['menus'])}")


if __name__ == "__main__":
    main()
