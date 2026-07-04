"""แก้ P3 (data): ปรับ energy tag เมนูกะทิเข้มข้น/ของหวานหนัก จาก 'กลาง' → 'สูง'

หลักเกณฑ์: มีกะทิเป็นแหล่งพลังงานหลัก, หรือแป้ง+น้ำตาล+ไขมันเข้มข้น,
หรือผัดที่ใส่เส้น/ทะเล/น้ำมันเยอะ
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MENU_PATH = ROOT / "data/menu-db.json"

PROMOTE_TO_HIGH = {
    "hor-mok-bai-yor",        # ห่อหมกใบยอ — กะทิ+ปลา+ไข่ เข้มข้น
    "gaeng-buad-fak-thong",   # แกงบวดฟักทอง — กะทิ+น้ำตาล
    "wun-kati",               # วุ้นกะทิ — ชั้นกะทิเข้มข้น
    "kluay-buachi",           # กล้วยบวชชี — กะทิ+น้ำตาล
    "tako",                    # ตะโก้ — กะทิ+แป้ง+น้ำตาล
    "lod-chong-fak-thai",     # ลอดช่องแตงไทย — กะทิ+น้ำเชื่อม+แป้ง
    "sarim",                   # ซาหริ่ม — กะทิ+น้ำเชื่อม+แป้ง
    "ice-cream",               # ไอศกรีม — น้ำตาล+ไขมัน
    "bua-loy-nam-khing",      # บัวลอย — แป้ง+กะทิ+น้ำตาล
    "gaeng-wan-gai",          # แกงหวานไก่ — กะทิ+ไก่+น้ำตาล
    "phat-khii-mao-thale",    # ผัดขี้เมาทะเล — ผัดน้ำมัน+เส้น+ทะเล
}


def main():
    db = json.loads(MENU_PATH.read_text(encoding="utf-8"))
    changed = []
    for m in db["menus"]:
        if m["id"] in PROMOTE_TO_HIGH and m.get("energy") != "สูง":
            old = m.get("energy")
            m["energy"] = "สูง"
            changed.append((m["id"], m["name"], old))

    if not changed:
        print("[p3.2] ไม่มีเมนูต้องปรับ (idempotent — อาจแก้ไปแล้ว)")
        return

    MENU_PATH.write_text(json.dumps(db, ensure_ascii=False, indent=2), encoding="utf-8")

    print(f"[p3.2] ปรับ energy 'กลาง/ต่ำ' → 'สูง' ทั้งหมด {len(changed)} เมนู:")
    for id_, name, old in changed:
        print(f"  {id_:<32} {name:<40} {old} → สูง")

    from collections import Counter
    dist = Counter(m.get("energy") for m in db["menus"])
    print(f"\n[p3.2] energy distribution หลังแก้: {dict(dist)}")


if __name__ == "__main__":
    main()
