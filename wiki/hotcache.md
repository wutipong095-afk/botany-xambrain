---
title: "Hotcache"
type: meta
created: 2026-07-03
updated: 2026-08-23
tags:
  - meta
---

# Hotcache

## สถานะปัจจุบัน

- **Phase**: **การทำสมองที่สอง** — Layer S (+S8 taxonomy · +M1–M14 monographs รายชนิด) · Layer T ครบ · U1/U2/U3 · AX1+AX2 · เครื่องแนะนำเมนู + wiki · **AI ติวเตอร์ (RAG) MVP ในแอป**
- **แกนความรู้ (ครูเฮนรี่)**: AX1 "อาหาร = สมดุล" → [[food-as-balance]] · AX2 "เคลื่อนไหว + ความสุขจริง vs หลอก" → [[human-unnatural-life]]
- **ทิศทางใหม่**: ทำเป็น "ฐานข้อมูล+เครื่องแนะนำเมนู" สำหรับประชาชน (ป้อน BMI+อาการ→เมนู) · ข้อมูลอยู่ `data/*.json` · เฟสถัดไป = เว็บ prototype
- **เป้าหมายถัดไป**: ทำข้อสอบ qbank ให้ monographs M1–M8 · วิเคราะห์เมนู Tier 2 รอบถัดไป · เติม kcal · ตัวกรองแพ้อาหาร · ผลิตข้อสอบพิมพ์จาก qbank taxonomy (ถ้าต้องการ) · (เมื่อเข้าถึงได้) ย่อยเล่มสมพร
- **ระบบอ้างอิง**: มี [[reference-sources]] แยก certified ✅ / external 🌐 — external ต้องมี URL เสมอ · มี **Textbook queue**
- **Pipeline ย่อยความรู้**: [[knowledge-extraction-pipeline]] — ย่อย → ตรวจอักษร (`scripts/check-text.py`) → ตัดภาพ (`scripts/extract-images.py`)
- **เว็บแนะนำเมนู**: `index.html` + `recommender.js` อ่าน `data/*.json` · ฐาน **124 เมนู** (Tier 2 = 28)
- **ตัดสินใจแล้ว**:
  - โครงสร้าง + workflow เหมือน body-xambrain (ดู CLAUDE.md)
  - Framework 3 Layer: S=พฤกษศาสตร์ / U=การใช้ประโยชน์ / T=ภูมิปัญญาไทย
  - Botany Literacy 5 ระดับ: Naming → Identifying → Using → Connecting → Conserving
  - ข้อสอบ 3 ระดับ: L1 จำ/ระบุ, L2 เชื่อมโยง, L3 วินิจฉัย/ประยุกต์
  - taxonomy เริ่มจากแหล่งเปิด (DNP · THP · มหิดล · SciMath) — เล่มสมพรเป็นคิวอัปเกรดเมื่อเข้าถึง ARU ได้

## Last Session (2026-09-15)

- **ชุดใหม่: 21 ชนิดจากใบงานนักเรียน** (`Downloads/สันฐาน/`) — 3 กลุ่ม · เริ่มทำเฉพาะชนิดที่มี PDF ต้นฉบับก่อน · เต็ม monograph เหมือน M1–M8 · branch `content/plant-profiles-pdf-batch`
  - **M9 [[plant-khun]]** คูน/ราชพฤกษ์ *Cassia fistula* (Fabaceae) — **แหล่งหลัก = ใบงานกลุ่มต้นคูน** เติมโครง M8: §0 data sheet → Naming → ราก/ลำต้น/ใบ/ดอก/ผล → Layer U/T → Misconceptions
  - **จุดเรียนหลัก**: วงศ์ถั่วเหมือน M2/M3/M8 **แต่คนละวงศ์ย่อย (Caesalpinioideae)** → ดอก **ไม่ใช่รูปดอกถั่ว** (5 กลีบแผ่กาง) · ฝัก **ทรงกระบอกไม่แตก (indehiscent)** ต่างจากฝักอัญชัน/แคที่แตก · เกสร 10 อัน **ขนาดไม่เท่ากัน** · ไม้ประจำชาติไทย · เนื้อในฝักเป็นยาระบาย
  - **อัปเดต**: [[index]] (M9) · [[reference-sources]] (+1 แถว) · back-link จาก [[plant-khae-ban]]
  - **M10 [[plant-somlom]]** ส้มลม *Aganonerion polymorphum* (**Apocynaceae — วงศ์ใหม่ตัวแรก**) — แหล่งหลัก = ใบงานส้มลม (ณิชาภา ภู่จีน)
    - **จุดเรียนหลัก**: **น้ำยางขาว** (ลักษณะวงศ์) · **ใบเดี่ยวเรียงตรงข้าม** (ตัดกับใบประกอบสลับของวงศ์ถั่ว) · **กลีบดอกเชื่อมเป็นหลอด** · **ผล follicle เป็นคู่ + เมล็ดมีปุยปลิวลม** → ตาราง**เทียบชนิดผล+การกระจายเมล็ด** (follicle/ลม ↔ legume คูน/สัตว์ ↔ legume อัญชัน/ดีดเอง) · ผักรสเปรี้ยว (lá giang)
    - **อัปเดต**: [[index]] (M10) · [[reference-sources]] (+1) · back-link จาก [[plant-khun]]
  - **M11 [[plant-namjaikhrai]]** น้ำใจใคร่ *Olax psittacorum* (**Olacaceae — วงศ์ใหม่ตัวที่สอง**) — แหล่งหลัก = ใบงานน้ำใจใคร่
    - **จุดเรียนหลัก**: **ดอกวง 3** (กลีบดอก 3 · เกสรผู้ 3 · ยอดเกสร 3 แฉก) **แต่กลีบเลี้ยง 5 รูปถ้วย** → ฝึกนับ merosity · **ผลกลมเมล็ดเดียว** (เติมตารางเทียบชนิดผล 4 ชนิด) · **หนึ่งชนิดหลายชื่อท้องถิ่น** → เคส binomial · เคสชื่อชน "กระทกรก" ซ้ำกับ *Passiflora foetida*
    - **อัปเดต**: [[index]] (M11) · [[reference-sources]] (+1) · back-link จาก [[plant-somlom]]
  - **M12 [[plant-maduea-chumphon]]** มะเดื่อชุมพร/อุทุมพร *Ficus racemosa* (**Moraceae — วงศ์เดียวกับหม่อน M4**) — แหล่งหลัก = ใบงานมะเดื่อ (มยุรี ผิวลมูล)
    - **จุดเรียนหลัก**: ช่อดอก **syconium (ฐานรองดอกปิด · ดอกซ่อนภายใน)** → ตารางเทียบผลรวม **syconium ↔ sorosis (หม่อน)** · **cauliflory** · **แตนมะเดื่อ (fig wasp)** · สะพาน S→T: **อุทุมพร/สำนวน "เห็นดอกมะเดื่อ"** อธิบายด้วยดอกที่ซ่อนใน syconium
    - **อัปเดต**: [[index]] (M12) · [[reference-sources]] (+1) · back-link จาก [[plant-mon]]
  - **M13 [[plant-supphannika]]** สุพรรณิการ์/ฝ้ายคำ *Cochlospermum regium* (**Bixaceae, Malvales**) + **M14 [[plant-chang]]** จั๋ง *Rhapis subtilis* (**Arecaceae — monocot ตัวที่ 2**) — มาจาก PDF ลายมือไฟล์เดียว (2 ชนิด) ทำพร้อมกัน commit เดียว
    - **สุพรรณิการ์**: ใบแฉกฝ่ามือ · ดอกเหลือง 5 กลีบ **อับเรณูเปิดรู → buzz pollination** · แคปซูล เมล็ดมีปุยฝ้าย (ชื่อฝ้ายคำ) · ⚠️ ไทยมักเป็น *C. religiosum*
    - **จั๋ง**: ปาล์มแตกกอ · รากฝอย · ใบรูปพัด เส้นใบขนาน · **ดอกวง 3 (3+3+6+3)** · berry → **ตารางเทียบ dicot ↔ monocot** ในโหนดจั๋ง
    - **อัปเดต**: [[index]] (M13, M14) · [[reference-sources]] (+2) · back-link จาก [[plant-khao-kae]] (monocot อีกตัว)
- **คิว PDF ถัดไป**: ประดู่ (`ตารางคำตอบต้นประดู่ป่า_แบ่งตามกลุ่ม.xlsx`) → จบคิว PDF · แล้วค่อยชนิดที่เหลือใน 21 (ไม่มีไฟล์)
- **หมายเหตุ**: "แค" (กลุ่ม 1) น่าจะซ้ำกับ [[plant-khae-ban]] (M8) — เหลือชนิดใหม่จริง ~20

## Last Session (2026-08-23)

- **M8 [[plant-khae-ban]]** แคบ้าน *Sesbania grandiflora* (Fabaceae) — โครงเดียวกับ M1–M7: **§0 ตารางข้อมูล (data sheet) 50 แถว** → Naming → ราก/ลำต้น/ใบ/ดอก/ผล → key แยกจากแคนา-โสน → Layer U/T → Misconceptions → ใบงานภาคสนาม
  - **จุดเด่น**: ดอกถั่วใหญ่ที่สุดในพืชอาหารไทย (ผ่าดูกลีบ+เกสรด้วยตาเปล่า) · **เกสรเพศผู้ 10 อัน (9)+1 diadelphous** · ใบขนนก **ปลายคู่ (paripinnate)** ตรงข้ามอัญชัน/เถาวัลย์เปรียงที่ปลายคี่ · **ปมรากตรึงไนโตรเจน** · nyctinasty
  - **เคสตรวจสอบข้อมูล**: เอกสารไทยที่ลอกต่อกันมาระบุ "เกสรเพศผู้ 60 อัน" (จริง = 10) และ "ฝักยาว 8–15 ซม." (ฝักแก่จริง 30–50 ซม. · 8–15 ซม. = ฝักอ่อนที่เก็บกิน) → ใช้สอน Botany Literacy ระดับ Identifying
  - **สะพาน S→U→T**: เกสรที่แม่ครัวเด็ดทิ้งก่อนแกงส้ม = androecium+gynoecium ที่ขม · ฤดูออกดอก (ปลายฝน–ต้นหนาว) ตรงฤดู **ไข้หัวลม** ที่ใช้ดอกแคแก้ · เปลือกรสฝาด (แทนนิน) แก้ท้องร่วง
  - **หมายเหตุ taxonomy**: PAPILIONACEAE = ชื่อเดิม → ปัจจุบัน Fabaceae วงศ์ย่อย Faboideae · ผู้ตั้งชื่อระบุไม่ตรงกัน (Poir./Desv./Pers.) *ยังไม่ยืนยัน*
- **Glossary**: +9 ศัพท์เข้า [[glossary/glossary-morphology]] (ปมราก · velamen · paripinnate · nyctinasty · standard/vexillum · wing · keel · campanulate · ornithophily)
- **อัปเดต**: [[index]] (M8) · [[reference-sources]] (+1 แถว external log 9 แหล่ง) · ลิงก์จาก [[plant-morphology]] · [[thai-herbal]]
- **ยังไม่ทำ**: qbank ของ M1–M8 · เพิ่มแคบ้านเข้า [[plant-morphology-comparison]] / [[plant-key-6species]] (จะกลายเป็น 8 ชนิด) · เชื่อมเมนูดอกแค (แกงส้มดอกแค) เข้า `data/menu-db.json`

## Last Session (2026-08-20)

- **เปิดหมวดใหม่: Plant monographs (เคสระดับชนิด M1–M6)** — โครงเดียวกันทุกโหนด: Naming → ราก/ลำต้น/ใบ/ดอก/ผล → Layer U/T → Misconceptions → ที่มา (external มี URL ครบ)
  - **M1 [[plant-cannabis]]** กัญชา — ใบประกอบนิ้วมือ · phyllotaxy เปลี่ยนตามตำแหน่ง · dioecious ผสมโดยลม · achene · **ไตรโครม 3 แบบ** (สะพาน S→U→กฎหมาย) · sativa/indica/ruderalis · กัญชา vs กัญชง · ประกาศ สธ. สมุนไพรควบคุม (กัญชา) พ.ศ. 2568 · 16 ตำรับที่มีกัญชาปรุงผสม
  - **M2 [[plant-anchan]]** อัญชัน — ขนนกปลายคี่ · ดอกรูปดอกถั่ว **resupinate** · ฝักแตก 2 แนว · ternatin/pH · ข้อควรระวังกับยาต้านเกล็ดเลือด
  - **M3 [[plant-thaowan-priang]]** เถาวัลย์เปรียง — ไม้เถาเนื้อแข็ง · เครื่องยาจาก**เถา** · รสเฝื่อนเอียน · บัญชียาหลัก · หลักฐานเทียบ diclofenac/ibuprofen
  - **M4 [[plant-mon]]** หม่อน — ยางขาว (Moraceae) · ใบ trinerved + หลายรูป · ช่อหางกระรอก · **ผลรวม sorosis** · ใบรสจืดเย็น
  - **M5 [[plant-tabaek]]** ตะแบกนา · **M6 [[plant-inthanin-bok]]** อินทนิลบก — ใช้ **เปลือกต้น** เป็นลักษณะวินิจฉัย + ตารางแยก 4 ชนิด (ตะแบก · เสลา · อินทนิลน้ำ · อินทนิลบก)
- **Glossary**: เติม 26 ศัพท์เข้า [[glossary/glossary-morphology]] (trichome · bract · raceme/panicle · papilionaceous · resupinate · hypanthium · achene · capsule · legume · sorosis · buttress · lenticel ฯลฯ) + แก้ลิงก์ glossary-herbal ที่เขียนว่า "ยังไม่สร้าง"
- **อัปเดต**: [[index]] (หมวด Plant monographs) · [[reference-sources]] (+6 แถว external log) · ลิงก์จาก [[plant-morphology]] · [[thai-herbal]]
- **ใบงาน 6 ชนิด**: [[plant-morphology-comparison]] (M0 ตารางสรุป) · [[plant-key-6species]] (K1 รูปวิธาน) · [[plant-classification-chart]] (K2 ผังอนุกรมวิธาน)
- **ผังกล่อง-ลูกศร**: `scripts/make-plant-charts.py` สร้าง SVG → แปลงเป็น PNG ด้วย LibreOffice → `assets/charts/` (ใช้ในโหนดและใน .docx)
- **เอกสารส่ง**: `output/worksheet-plant-morphology-6species.docx` (11 หน้า A4 แนวนอน: ตาราง · รูปวิธาน · ผังจำแนก · ผังอนุกรมวิธาน) สร้างด้วย docx-js
- **ยังไม่ทำ**: qbank ของ M1–M6 · ตัดภาพประกอบ (ยังไม่มีไฟล์ใน `raw/`) · เชื่อม M4 หม่อนเข้าฐานเมนู

## Last Session (2026-08-09)

- **Textbook queue**: บันทึกเล่ม *การตรวจเอกลักษณ์พืชสมุนไพร : พฤกษอนุกรมวิธาน* (สมพร ภูติยานันต์, จุฬาฯ) ใน [[reference-sources]] — ARU Hibrary ต้อง `@aru.ac.th` (ยังย่อยตรงไม่ได้)
- **แหล่งเปิด taxonomy**: DNP ชื่อพรรณไม้ · THP · รูปวิธาน/ Siree มหิดล · SciMath Zingiberaceae → log ใน [[reference-sources]]
- **S8 [[plant-taxonomy]]** scaffold 🌐 + [[glossary/glossary-taxonomy]] ✅ · อัปเดต [[index]] · [[glossary/glossary-index]] · ลิงก์จาก [[plant-morphology]] · [[thai-herbal]] · [[overview]]
- **ขยาย S8 §7**: เคส THP 5 ชนิด (ขิง · ขมิ้นชัน · กระชาย · ฟ้าทะลายโจร · มะขามป้อม) + sync ศัพท์ glossary
- **ข้อสอบ**: [[questions/qbank-plant-taxonomy]] 12 ข้อ (L1×5 · L2×4 · L3×3) + [[assessment/master-assessment]]
- **[[glossary/glossary-herbal]]** ✅ รสยา 9+1 · ธาตุ 4 · พิกัด (บาลี/สันสกฤต + วิธีอ่าน)
- **เว็บ**: `index.html` แสดงสถานะโหลดจำนวนเมนูใน footer · ยืนยันฐาน 124 เมนู (เป้า 50+ ครบแล้ว)
- **AI tutor**: PR #5 `feature/ai-tutor-chat` **merged แล้ว** (ไม่ต้องเปิดใหม่)

## Last Session (2026-07-06)

- **AI ติวเตอร์ (RAG) ในแอปครู** — branch `feature/ai-tutor-chat`
- **Backend**: `app/src-tauri/src/ai.rs` — Gemini 2.5 Flash + embedding-001 · cosine top-5 · async commands · API key ใน config dir
- **Frontend**: `app/src/chat.ts` — พาเนลแชตขวา · modal ตั้ง key · ชิปอ้างอิงคลิกเปิดบทเรียน
- **Build index**: `scripts/build-embeddings.py` → `data/embeddings.json` (257 chunks, gitignore) · resume + rate-limit aware
- **แก้บั๊ก**: UTF-8 truncate · Mutex drop ก่อน network · modal fixed overlay · maxOutputTokens 8192
- **Docs**: ARCHITECTURE §7 · app/README (วิธีตั้ง key + build embeddings)

## Last Session (2026-07-05)

- **ย่อย PDF ใหม่**: `ไม้ดอก1.pdf` (29 ห.) → [[source-maidok1]]
- **ตัดภาพ**: `assets/ไม้ดอก1/` (76 ไฟล์) · catalog [[catalog-maidok1]]
- **ขยาย S5 [[flower-morphology]]**: ก้านดอกย่อย/ฐานรอง · สมมาตร (ตัวอย่างชบา-บัว-อัญชัน) · ความครบ vs สมบูรณ์เพศ · ช่อดอก 12 แบบ + racemose/cymose · รูปดอก/perianth เสริม
- **sync ศัพท์**: ก้านดอกย่อย · ช่อแบบหาง · dichlamydeous/homochlamydeous เข้า glossary

## Last Session (2026-07-04)

- **ย่อย PDF ใหม่ 2 ไฟล์**: `ดอก ผล เมล็ด.pdf` (7 ห.) → [[source-flower-fruit-seed]] · `สัณฐานวิทยา_ใบ.pdf` (58 ห.) → [[source-leaf-extended]]
- **อัพเดต S5/S6**: [[flower-morphology]] · [[fruit-seed-morphology]] จาก external → **mixed** (certified + สสวท.)
- **ขยาย S4**: [[leaf-morphology]] — 6 ประเภทใบ · ใบดัดแปลง 10 · ผิว/เนื้อใบ
- **ตัดภาพ**: `assets/ดอก-ผล-เมล็ด/` (14 JPEG) · catalog [[catalog-flower-fruit-seed]]
- **แก้ path รูป**: wikilink ใช้ `assets/...` ให้ Obsidian แสดงรูปได้
- **ขยาย S5 [[flower-morphology]]**: เพิ่มสมมาตรดอก · การเชื่อม/แยกกลีบ · perianth/tepal · ตำแหน่งรังไข่ (hypo/peri/epigynous) · เพศพืช (monoecious/dioecious) · การผสมเกสร · ลักษณะดอก 6 วงศ์ · ตาราง Layer U/T (อาหาร/รสยา/ย้อมสี/พิธีกรรม) — sync ศัพท์ 13 คำเข้า glossary · เขียนไฟล์ LF ล้วน (แก้ CRLF)

## Last Session (2026-07-03)

- **สร้างสาขาอาหารครบชุด**: U1 [[food-dhatu-plants]] · U2 [[food-analysis-ttm]] · U3 [[food-recommender]] · แกน AX1 [[food-as-balance]]
- **U2 สะพาน "แคลอรี ↔ ธาตุไฟ (ปริณามัคคี)"** — วางเป็นสะพานการศึกษา + เมทริกซ์ 2×2 (ร้อน-เย็น × พลังงาน) + เคส 3 เมนู
- **U3 เครื่องแนะนำเมนู** — ฐานข้อมูล `data/menu-db.json` (13 เมนู) + `data/symptom-element.json` (10 กฎ) + กลไก BMI+อาการ→เมนู + prototype เดโมทำงานจริง
- **AX1 (ครูเฮนรี่) "อาหาร=สมดุล"** — เทียบ 4 กระบวนทัศน์ (สากล/ไทย/อายุรเวท/จีน) + บรรณานุกรมจริง
- ยืนยันวิชาจริงรองรับ: **TA301** (ม.ธรรมศาสตร์) → หมวด Curriculum anchors ใน [[reference-sources]]
- บันทึก memory (project): แกนอาหาร=สมดุล + เป้าหมายเครื่องแนะนำเมนู

## สิ่งที่ทำแล้ว

1. ~~Scaffold โปรเจกต์ + framework~~ ✅
2. ~~ย่อยเนื้อหา สัณฐานวิทยา (ราก/ลำต้น/ใบ)~~ ✅
3. ~~ตัดต่อภาพใบ (8 รูป) + ถอดเนื้อหาเข้า [[leaf-morphology]]~~ ✅
4. ~~วางระบบอ้างอิง [[reference-sources]] + node ดอก/ผล-เมล็ด (external, มี URL)~~ ✅
5. ~~เริ่ม Layer T: [[thai-herbal]] hub + [[herbal-taste-9]] (external, อ้างอิง ม.อ.)~~ ✅
6. ~~[[dhatu-4-plants]]: ธาตุ 4 + รสแก้ธาตุ + เบญจกูล เชื่อม Layer S ครบวง~~ ✅
7. ~~[[herbal-formula]] ตำรับ ตรีกฏุก/ตรีผลา/เบญจกูล (external)~~ ✅
8. ~~glossary-morphology (ไทย/อังกฤษ/รากศัพท์) + glossary-index~~ ✅
9. ~~[[plant-tissue]] (S7) เนื้อเยื่อเจริญ/ถาวร + xylem/phloem (external, สสวท.)~~ ✅
10. ~~เริ่ม Layer U: [[food-dhatu-plants]] (U1) — ผัก–ผลไม้ที่คนไทยกิน รสตามธาตุเจ้าเรือน (external 4 แหล่งสถาบัน) เชื่อม Layer S/T ครบวง~~ ✅
11. ~~[[food-analysis-ttm]] (U2) — วิเคราะห์อาหาร รส-ฤทธิ์-พลังงาน (สะพานแคลอรี↔ธาตุไฟ)~~ ✅
12. ~~[[food-recommender]] (U3) + `data/*.json` — เครื่องแนะนำเมนู (BMI+อาการ→เมนู) + prototype~~ ✅
13. ~~[[food-as-balance]] (AX1) — แกนความรู้ อาหาร=สมดุล เทียบ 4 กระบวนทัศน์ (ครูเฮนรี่)~~ ✅
14. ~~[[human-unnatural-life]] (AX2) — แกนความรู้ ชีวิตผิดธรรมชาติ: เคลื่อนไหว + โดพามีน/ความสุขหลอก~~ ✅
15. ~~ย่อย `ดอก ผล เมล็ด.pdf` + อัพเดต S5/S6 (mixed certified)~~ ✅
16. ~~ย่อย `สัณฐานวิทยา_ใบ.pdf` 58 ห. + ขยาย S4~~ ✅
17. ~~ย่อย `ไม้ดอก1.pdf` 29 ห. + ขยาย S5 (certified ฉบับเต็มดอก)~~ ✅
18. ~~AI ติวเตอร์ RAG ใน Teacher App (Gemini + embeddings + chat panel)~~ ✅
19. ~~S8 [[plant-taxonomy]] scaffold + [[glossary/glossary-taxonomy]] จากแหล่งเปิด~~ ✅
20. ~~Textbook queue: สมพร ภูติยานันต์ (ARU Hibrary)~~ ✅
21. ~~S8 §7 เคส THP 5 ชนิด (ขิง/ขมิ้น/กระชาย/ฟ้าทะลาย/มะขามป้อม)~~ ✅
22. ~~ข้อสอบ [[questions/qbank-plant-taxonomy]] (12 ข้อ)~~ ✅
23. ~~[[glossary/glossary-herbal]] (บาลี/สันสกฤต)~~ ✅
24. ~~เว็บแนะนำเมนู: footer สถานะโหลด · ฐาน 124 เมนู~~ ✅

## ต้องทำต่อ

1. **PR** branch `content/reference-putiyanan-herbal-id` → master
2. วิเคราะห์เมนู Tier 2 รอบ 2–5 · เติม kcal · ตัวกรองแพ้อาหาร/โรคประจำตัว
3. Deploy เว็บแนะนำเมนูให้ประชาชนเข้าถึงได้ (hosting)
4. (ต่อยอด) node เจาะลึก อายุรเวท / แพทย์แผนจีน เทียบเชิงลึก
5. **ข้อสอบ concept อื่น** — ยังพักชุดใหญ่ · มี qbank taxonomy แล้ว
6. **AI chat ต่อ**: command ออกข้อสอบเฉพาะ · เพิ่ม top-N สำหรับคำถามกว้าง · (ทางเลือก) Gemini Pro
7. (เมื่อเข้าถึงได้) ย่อยเล่มสมพรจาก Textbook queue
8. (ทางเลือก) ผลิตข้อสอบพิมพ์จาก qbank taxonomy → `output/`

## Open Questions

- เนื้อหาตั้งต้นจะมาจากแหล่งไหน? (ตำราเรียน / slide อาจารย์ / งานภาคสนาม)
- ขอบเขตพืช: เน้นสมุนไพร หรือครอบคลุมพืชอาหาร+เครื่องใช้+ย้อมสีด้วย?
