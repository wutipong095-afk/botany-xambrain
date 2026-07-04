---
title: "แหล่งอ้างอิงและระดับการรับรอง"
type: reference
created: 2026-07-03
updated: 2026-07-03
tags:
  - meta
  - reference
---

# แหล่งอ้างอิงและระดับการรับรอง (Source Provenance)

เพื่อความน่าเชื่อถือ ทุก concept node ในโปรเจกต์ระบุ **ระดับการรับรองของแหล่งข้อมูล** ผ่านฟิลด์ `source_type` ใน frontmatter

## ระดับแหล่งข้อมูล

| `source_type` | ความหมาย | สัญลักษณ์ในเนื้อหา |
|---------------|----------|---------------------|
| **certified** | จากซีทสอน/เอกสารอาจารย์ (รับรองแล้ว) | ✅ |
| **external** | ค้นจากอินเทอร์เน็ต/ตำราภายนอก — ต้องมี URL อ้างอิงเสมอ | 🌐 |
| **mixed** | ผสม certified + external (แยกส่วนให้ชัดในเนื้อหา) | ✅🌐 |

## กติกา

1. เนื้อหาจากซีทอาจารย์ = **certified** — เป็นแกนหลักที่เชื่อถือได้
2. เนื้อหาที่ **ค้นจากอินเทอร์เน็ต** ต้อง:
   - ใส่ `source_type: external` (หรือ mixed)
   - มีหัวข้อ **"ที่มา (External sources)"** ท้าย node พร้อม URL แบบ markdown link
   - ระบุวันที่เข้าถึง
3. เลือกแหล่งที่น่าเชื่อถือก่อน: สสวท. (scimath.org), หน่วยงานรัฐ/สถาบันการศึกษา (.ac.th, .go.th), ตำราชีววิทยา
4. ห้ามแต่งข้อมูล/URL — ถ้าไม่พบแหล่งยืนยัน ให้ระบุว่า "ยังไม่ยืนยัน"

## Certified sources (ซีทอาจารย์)

- [[source-plant-morphology]] — สัณฐานวิทยาพืช: ราก ลำต้น ใบ (วัชราภรณ์ ชนะเคน, วชช.มุกดาหาร)
- [[source-leaf-extended]] — สัณฐานวิทยาใบ ฉบับเต็ม 58 หน้า (วชช.มุกดาหาร)
- [[source-flower-fruit-seed]] — ดอก ผล เมล็ด 7 หน้า (วชช.มุกดาหาร)

## Curriculum anchors (วิชา/หลักสูตรทางการที่รองรับ)

แหล่งระดับ "หลักสูตร" — ใช้ยืนยันว่าหัวข้อในโปรเจกต์มีวิชาจริงรองรับ (สูงกว่าบทความสุขภาพทั่วไป)

| รหัส/วิชา | สถาบัน | ขอบเขต | รองรับ node |
|-----------|--------|--------|-------------|
| **TA301 Food and Nutrition for Applied Thai Traditional Medicine** | ม.ธรรมศาสตร์ (หลักสูตรแพทย์แผนไทยประยุกต์) · อ.อรุณพร อิฐรัตน์ และคณะ | อาหาร/สารอาหาร · การย่อย-ดูดซึม · ประเมินภาวะโภชนาการ · **อาหารตามธาตุเจ้าเรือน** (บูรณาการโภชนาการสมัยใหม่ + แผนไทย) | [[food-dhatu-plants]] (U1) · [[food-analysis-ttm]] (U2) · [[food-recommender]] (U3) |

> 📎 TA301 = anchor ยืนยันว่า "โภชนบำบัดตามธาตุเจ้าเรือน" เป็นศาสตร์ในหลักสูตรจริง และวิชาจริง **บูรณาการ** โภชนาการสมัยใหม่กับธาตุ — ตรงกับแนวสะพาน "แคลอรี↔ธาตุไฟ" ใน U2
> ที่มา: [รายวิชาหลักสูตรแพทย์แผนไทยประยุกต์ — TU Moodle](https://moodle.tu.ac.th/course/index.php?categoryid=1253) · [โภชนบำบัดตามธาตุเจ้าเรือน — TMJ (TCI)](https://he02.tci-thaijo.org/index.php/tmj/article/download/243873/165769)

## External source log (แหล่งอินเทอร์เน็ตที่ใช้แล้ว)

| วันที่ | หัวข้อ | แหล่ง | ใช้ใน node |
|--------|--------|-------|-----------|
| 2026-07-03 | ส่วนประกอบ/ชนิดของดอก | สสวท. scimath.org, ทรูปลูกปัญญา | [[flower-morphology]] |
| 2026-07-03 | ชนิดของผล + เมล็ด | สสวท. scimath.org, ทรูปลูกปัญญา | [[fruit-seed-morphology]] |
| 2026-07-03 | รสยา 9 รส + สรรพคุณ | คณะการแพทย์แผนไทย ม.อ. (ttmed.psu.ac.th), Poonrada | [[herbal-taste-9]], [[thai-herbal]] |
| 2026-07-03 | ธาตุ 4 + รสแก้ธาตุ + เบญจกูล | ม.มหิดล (pharmacy), เภสัชกรรมไทย (samunpri), be7herb | [[dhatu-4-plants]] |
| 2026-07-03 | ตำรับ ตรีกฏุก/ตรีผลา/เบญจกูล | ม.สงขลานครินทร์ (ttmed 197), samunpri, be7herb | [[herbal-formula]] |
| 2026-07-03 | เนื้อเยื่อพืช (เจริญ/ถาวร, xylem/phloem) | สสวท. (scimath 7031), วิกิพีเดียไทย | [[plant-tissue]] |
| 2026-07-03 | อาหารตามธาตุเจ้าเรือน: ผัก–ผลไม้ รสตามธาตุ | ม.มหิดล กาญจนาภิเษก (gj.mahidol), ราชวิทยาลัยจุฬาภรณ์ (cra.ac.th), สวพส. (hrdi), รพ.พิษณุโลก | [[food-dhatu-plants]] |
| 2026-07-03 | เตโชธาตุ/ไฟ 4 กอง + ฤทธิ์ร้อน-เย็น (สะพานแคลอรี↔ธาตุไฟ) | ม.มหิดล (pharmacy 98), รพ.พริ้นซ์ (princhealth), คลินิกพังงา | [[food-analysis-ttm]] |
| 2026-07-03 | อาหารคือสมดุล: เปรียบเทียบ 4 กระบวนทัศน์ + บรรณานุกรม | WHO (healthy diet), FAO (FBDG), ม.สงขลานครินทร์ (ttmed 97 อายุรเวท), รพ.รามา (mahidol) | [[food-as-balance]] |
| 2026-07-03 | หลักฐานเชิงประจักษ์: อาหาร→โรคทั่วโลก (11 ล้าน/22%, NCDs 74%) | GBD 2017 *The Lancet*, EurekAlert, WHO (NCDs, EMRO) | [[food-as-balance]] |
| 2026-07-03 | เมนูอาหารตามธาตุ (+14 เมนู): ลม/น้ำ/ไฟ/ดิน | ม.มหิดล gj.mahidol (tard, preg-food), ราชวิทยาลัยจุฬาภรณ์ cra.ac.th | `data/menu-db.json` · [[food-recommender]] |
| 2026-07-03 | เมนูอาหารตามธาตุ รอบ 2 (+24 เมนู → 50 รวม): ดิน/ลม/ไฟ/น้ำ | ม.มหิดล gj.mahidol (tard, preg-food), ราชวิทยาลัยจุฬาภรณ์ cra.ac.th | `data/menu-db.json` · [[food-recommender]] |
| 2026-07-03 | เมนูผู้ป่วย/ฟื้นตัว (+12 เมนู, patientFor) + กฎอาการผู้ป่วย | ม.มหิดล gj (chemo, กล้วยน้ำว้า), ศรีพัฒน์ CMU, ศิริราช si/rama, ม.สวนดุสิต | `data/menu-db.json` · `data/symptom-element.json` |
| 2026-07-03 | อายุสมุฏฐาน 4 วัย → ธาตุ/รสตามวัย (`age-food.json`) | ม.เชียงใหม่ med.cmu, ม.มหิดล gj, samunpri, สารานุกรมไทย | `data/age-food.json` · [[food-recommender]] · `recommender.js` |
| 2026-07-03 | เมนูอาหารรอบ 3 (+14 เมนู → 77 รวม): ของว่าง/เครื่องดื่มธาตุดิน-น้ำ-ไฟ | ราชวิทยาลัยจุฬาภรณ์ cra.ac.th, ม.มหิดล gj (miang-kham, okra, preg-food) | `data/menu-db.json` · [[food-recommender]] |
| 2026-07-03 | เมนูอาหารรอบ 4 (+15 เมนู → 92 รวม): เครื่องดื่ม CRA + อาหารอ่อนผู้ป่วย | cra.ac.th, gj.mahidol, ศรีพัฒน์ CMU (sriphat) | `data/menu-db.json` · [[food-recommender]] |
| 2026-07-03 | เมนูอีสานคัดกรองสุขภาพ (+14 ใหม่, 4 แท็ก region) สุก ไม่ปลาดิบ/ปลาร้า | prijnr 2023 (CMU), gj.mahidol, KKU Smart Aging, SUT ผักอีสาน | `data/menu-db.json` · [[food-recommender]] |
| 2026-07-03 | ยำสุขภาพ (+12 ใหม่, yumScreened) สัตว์สุก ไม่ปลาดิบ แคลอรีต่ำ | gj.mahidol, cra.ac.th, prijnr 2023, sriphat CMU | `data/menu-db.json` · [[food-recommender]] |
| 2026-07-03 | วิเคราะห์เมนู Tier 2 รอบ 1 (แกง/ต้ม 28 + แกงหน่อไม้) + UI ดูวิเคราะห์ | doctor.or.th, inmu.mahidol, gj.mahidol, cra.ac.th, prijnr 2023 | `data/menu-analysis-tier2-round1.json` · `wiki/menus/` · `recommender.js` |
| 2026-07-03 | มนุษย์ใช้ชีวิตผิดธรรมชาติ: เคลื่อนไหว + โดพามีน/ความสุขหลอก (AX2) | WHO (physical activity, NCDs), NIH/NIMH (reward system) | [[human-unnatural-life]] |
| 2026-07-04 | กฎอาการผู้ป่วย: ท้องเสีย(ฝาด) · คลื่นไส้/กลืนลำบาก(อาหารอ่อน) · ฟื้นตัวหลังไข้/ผ่าตัด | [ศูนย์ความเป็นเลิศเคมีบำบัด gj.mahidol](https://www.gj.mahidol.ac.th/main/knowledge-2/chemo-nutrition/) · [ศูนย์ศรีพัฒน์ CMU](https://sriphat.med.cmu.ac.th) · [ศิริราช si.mahidol](https://www.si.mahidol.ac.th) · [รพ.รามาธิบดี](https://www.rama.mahidol.ac.th) | `data/symptom-element.json` (กฎ 8, 11, 12, 13) |
| 2026-07-04 | ติดแท็กอาการเพิ่มให้เมนู (26 เมนู): กระหายน้ำมาก · ปัสสาวะเหลือง · ท้องผูกจากร้อน · บวมน้ำ · อ้วนง่าย · ตัวหนักเมื่อยล้า · ฟื้นตัว/หลังไข้/หลังผ่าตัด/อ่อนเพลียมาก | อ้างอิงเดิม (gj.mahidol, cra.ac.th, sriphat, si/rama) ดูแถวก่อนหน้า | `data/menu-db.json` |
| 2026-07-04 | ทำความสะอาด `recommendTaste` — ถอด "เย็น" (thermal ถือแล้ว) และ "สุขุม" (เป็นฤทธิ์ ไม่ใช่ 1 ใน 9 รส) | ตาม [[herbal-taste-9]] (ม.สงขลานครินทร์) | `data/symptom-element.json` (กฎ 2, 4, 5) |
| 2026-07-04 | ให้ scoring engine ใช้ `state` (กำเริบ/หย่อน) — หย่อน→บำรุงพลังงานสูง · กำเริบ→เลือกเมนูเบา/ระบาย | หลักธาตุ 4 แผนไทย ([[dhatu-4-plants]]) + TA301 (บำรุง vs ระบาย) | `recommender.js` (aggregateTargets + scoreMenu) |
| 2026-07-04 | ปรับ `energy` เมนูกะทิ/หวานเข้ม 11 เมนู กลาง→สูง (ห่อหมก, บวชชี, ตะโก้, ซาหริ่ม, ลอดช่อง, ไอศกรีม, ฯลฯ) — แก้ distribution สูง=3→14 | ตำรับกะทิเข้มข้น: [ราชวิทยาลัยจุฬาภรณ์](https://www.cra.ac.th) · [gj.mahidol](https://www.gj.mahidol.ac.th) | `data/menu-db.json` |
| 2026-07-04 | Symptom chips ใน `index.html` โหลด dynamic จาก `symptom-element.json` (เดิม hardcode 10) จัดกลุ่มตามธาตุ+state | ปรับตามฐานกฎจริง | `index.html` (buildSymptomGroups + buildChips) |
| 2026-07-04 | แก้ [[leaf-morphology]] ให้ตรง PDF หน้า 4 — เพิ่ม acuminate/lobed · แก้ retuse↔emarginate · โคนตัดแทน auriculate · ตาราง 9+8+8 แบบ | [[source-plant-morphology]] (วชช.มุกดาหาร, สไลด์ใบ p004-02) | `wiki/leaf-morphology.md` · `wiki/glossary/glossary-morphology.md` |
| 2026-07-04 | ย่อย `raw/ดอก ผล เมล็ด.pdf` (7 ห.) — ดอก/ช่อดอก/รูปดอก/เกสร/ผลสด-แห้ง · ตัดภาพ 14 ไฟล์ | วชช.มุกดาหาร (วัชราภรณ์ ชนะเคน) | [[source-flower-fruit-seed]] · [[flower-morphology]] · [[fruit-seed-morphology]] · `assets/ดอก-ผล-เมล็ด/` |
| 2026-07-04 | ย่อย `raw/สัณฐานวิทยา_ใบ.pdf` (58 ห.) — 6 ประเภทใบ · ใบดัดแปลง 10 · ผิว/เนื้อใบ · equitant | วชช.มุกดาหาร (Watchaaraporn Chanaken) + KU/Mahidol key (external ในสไลด์) | [[source-leaf-extended]] · [[leaf-morphology]] |
| 2026-07-04 | แก้ path รูป Obsidian — เติม prefix `assets/` ใน wikilink รูป | convention ใหม่ | `wiki/leaf-morphology.md` · `assets/catalog-leaf-morphology.md` |
