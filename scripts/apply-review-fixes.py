"""ใช้ครั้งเดียว: แก้ P1 (ติดแท็กอาการเมนู) + P2 (taste vocab) ตามผลรีวิว 2026-07-04

- P1: ติดแท็กอาการที่ไม่มีเมนูรับให้เมนูที่เหมาะ (3 กฎที่ MISS)
- P2: ถอน 'เย็น' และ 'สุขุม' ออกจาก recommendTaste (ไม่ใช่ 1 ใน 9 รส)

รันซ้ำได้ (idempotent) — ถ้าเมนูมีแท็กแล้วจะไม่เพิ่มซ้ำ
"""
import json
from pathlib import Path

root = Path(__file__).resolve().parent.parent
menu_path = root / "data/menu-db.json"
rule_path = root / "data/symptom-element.json"

# ---------- P1: ติดแท็กอาการ ----------
SYMPTOM_TAGS = {
    # ไฟ กำเริบ — กระหายน้ำมาก, ปัสสาวะเหลืองเข้ม, ท้องผูกจากร้อน
    "nam-taeng-mo": ["กระหายน้ำมาก", "ปัสสาวะเหลืองเข้ม"],
    "nam-maprao": ["กระหายน้ำมาก"],
    "nam-raak-bua": ["กระหายน้ำมาก"],
    "nam-yanang": ["กระหายน้ำมาก", "ปัสสาวะเหลืองเข้ม", "บวมน้ำ", "ตัวหนักเมื่อยล้า"],
    "nam-krachiap": ["ปัสสาวะเหลืองเข้ม", "ท้องผูกจากร้อน", "บวมน้ำ"],
    "nam-farang": ["ปัสสาวะเหลืองเข้ม", "ท้องผูกจากร้อน"],
    "nam-fak-khao": ["ปัสสาวะเหลืองเข้ม", "ท้องผูกจากร้อน"],
    "phat-phak-bung": ["ท้องผูกจากร้อน"],
    "lod-chong-fak-thai": ["ท้องผูกจากร้อน"],

    # น้ำ กำเริบ — บวมน้ำ, อ้วนง่าย, ตัวหนักเมื่อยล้า
    "gaeng-jued-tamlueng": ["บวมน้ำ"],
    "nam-bai-bua-bok": ["บวมน้ำ", "อ้วนง่าย"],
    "gaeng-liang": ["บวมน้ำ", "ตัวหนักเมื่อยล้า"],
    "mara-phat-khai": ["อ้วนง่าย", "ตัวหนักเมื่อยล้า"],
    "gaeng-khilek": ["อ้วนง่าย", "ตัวหนักเมื่อยล้า"],
    "phak-neung": ["อ้วนง่าย"],
    "gaeng-om-gai": ["อ้วนง่าย"],

    # ดิน หย่อน — ฟื้นตัว, หลังไข้, หลังผ่าตัด, อ่อนเพลียมาก
    "jok-pla-khing": ["ฟื้นตัว", "หลังไข้", "หลังผ่าตัด", "อ่อนเพลียมาก"],
    "jok-moo-sap-sai-khai": ["ฟื้นตัว", "หลังไข้", "หลังผ่าตัด", "อ่อนเพลียมาก"],
    "khao-tom-pla": ["ฟื้นตัว", "หลังไข้", "อ่อนเพลียมาก"],
    "khao-tom-kai": ["ฟื้นตัว", "หลังไข้", "อ่อนเพลียมาก"],
    "khao-tom-manao": ["ฟื้นตัว", "หลังไข้"],
    "sup-khao-sai": ["ฟื้นตัว", "หลังผ่าตัด", "อ่อนเพลียมาก"],
    "sup-fak-thong": ["ฟื้นตัว", "หลังผ่าตัด"],
    "sup-khao-phod-nom": ["ฟื้นตัว", "หลังผ่าตัด"],
    "gaeng-jued-woonsen": ["หลังผ่าตัด"],
    "gaeng-jued-thua-lek": ["ฟื้นตัว", "หลังผ่าตัด"],
}

db = json.loads(menu_path.read_text(encoding="utf-8"))
menu_index = {m["id"]: m for m in db["menus"]}

p1_report = []
missing_ids = []
for mid, new_tags in SYMPTOM_TAGS.items():
    m = menu_index.get(mid)
    if not m:
        missing_ids.append(mid)
        continue
    current = m.get("symptoms") or []
    added = [t for t in new_tags if t not in current]
    if added:
        m["symptoms"] = current + added
        p1_report.append((mid, added))

# ---------- P2: taste vocab ----------
rules_data = json.loads(rule_path.read_text(encoding="utf-8"))

REMOVE_TASTES = {"เย็น", "สุขุม"}
p2_report = []
for i, r in enumerate(rules_data["rules"]):
    tastes = r.get("recommendTaste", [])
    kept = [t for t in tastes if t not in REMOVE_TASTES]
    removed = [t for t in tastes if t in REMOVE_TASTES]
    if removed:
        r["recommendTaste"] = kept
        # เก็บบันทึกในความคิดเห็นของกฎ (why)
        note = " · หมายเหตุ: ถอด " + ",".join(removed) + " ออกจาก taste (thermal ถือ 'เย็น' แล้ว; 'สุขุม' เป็นฤทธิ์ ไม่ใช่ 1 ใน 9 รส)"
        if note not in r.get("why", ""):
            r["why"] = r["why"] + note
        p2_report.append((i, removed, kept))

# ---------- เขียนไฟล์ ----------
def dump(path: Path, data):
    path.write_text(
        json.dumps(data, ensure_ascii=False, indent=2) + "\n",
        encoding="utf-8",
    )

dump(menu_path, db)
dump(rule_path, rules_data)

# ---------- รายงาน ----------
print(f"== P1: menus tagged ({len(p1_report)}) ==")
for mid, tags in p1_report:
    print(f"  + {mid:<32}  {tags}")
if missing_ids:
    print(f"\n!! menu id ไม่พบ: {missing_ids}")

print(f"\n== P2: rules cleaned ({len(p2_report)}) ==")
for i, removed, kept in p2_report:
    print(f"  rule[{i}] removed={removed} → kept={kept}")
