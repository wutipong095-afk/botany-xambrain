# Botany-Xambrain Project

**ชื่อโปรเจกต์: การทำสมองที่สอง** — ระบบความรู้ภายนอก (externalized cognition) ด้านพฤกษศาสตร์พื้นบ้าน สุขภาพไทย และการใช้พืช

> โฟลเดอร์/repo ยังใช้ชื่อเทคนิค `botany-xambrain` · ชื่อเผยแพร่ = **การทำสมองที่สอง**

Obsidian vault สำหรับผู้เรียนพฤกษศาสตร์พื้นบ้าน — ผสานพฤกษศาสตร์สมัยใหม่กับภูมิปัญญาการใช้พืชแบบไทย

## โครงสร้างโฟลเดอร์

```
raw/          — PDF ต้นฉบับ (lecture slides, ตำรา, เอกสารภาคสนาม)
assets/       — รูปภาพที่ตัดจาก PDF + catalog
wiki/         — เนื้อหาหลัก (concept nodes, source summaries, questions, glossary, assessment)
```

## Framework

- **3 Layer**: Structure/พฤกษศาสตร์ (S) / Utilization/การใช้ประโยชน์ (U) / Thai Traditional/ภูมิปัญญาไทย (T)
- **Botany Literacy 5 ระดับ**: Naming → Identifying → Using → Connecting → Conserving
- **ข้อสอบ 3 ระดับ**: L1 จำ/ระบุ → L2 เชื่อมโยง → L3 วินิจฉัย/ประยุกต์

### รายละเอียด 3 Layer

- **Layer S — พฤกษศาสตร์ (Botany/Structure)**: สัณฐานวิทยา (morphology), การจำแนกพืช (taxonomy), ส่วนของพืช (ราก ลำต้น ใบ ดอก ผล เมล็ด), เนื้อเยื่อพืช
- **Layer U — การใช้ประโยชน์ (Utilization/Function)**: อาหาร, ยา, เครื่องใช้/หัตถกรรม, ย้อมสี, พิธีกรรม/ความเชื่อ, พืชเศรษฐกิจ
- **Layer T — ภูมิปัญญาไทย (Thai Traditional)**: สมุนไพรไทย, ตำรับยา, รสยา 9 รส, ธาตุ 4, สรรพคุณตามคัมภีร์, ความเชื่อและองค์ความรู้ท้องถิ่น

## Git

- **Remote**: GitHub — merge ผ่าน **Pull Request** เท่านั้น (ดู [`CONTRIBUTING.md`](CONTRIBUTING.md))
- **Branch**: ห้าม commit ตรง `master` — ใช้ `<type>/<name>` เช่น `content/`, `fix/`, `chore/`
- **CI**: GitHub Actions (`.github/workflows/ci.yml`) รัน `scripts/check-text.py` ทุก PR
- **Line endings**: LF ทั้ง repo (`.gitattributes`)
- **Ignore**: `.gitignore` — Obsidian workspace, OS files, `app/node_modules`, `output/`

## ชุดคำสั่ง (Workflows)

เมื่อผู้ใช้พิมพ์คำสั่งด้านล่าง ให้ทำตาม workflow ที่กำหนด:

### 1. ย่อยเนื้อหา `[ชื่อ PDF หรือ topic]`

**Input**: PDF ใน `raw/` หรือ topic ที่ต้องการ
**Output**: `wiki/source-*.md` + อัพเดต concept nodes

ขั้นตอน:
1. อ่าน PDF target (ถ้าเป็น image slides → อ่านชื่อไฟล์ + จำนวนหน้า + ดูรูปตัวอย่าง)
2. สรุปเนื้อหาแต่ละบท/section → สร้าง `wiki/source-{name}.md`
3. เชื่อมเนื้อหากับ concept nodes ที่มีอยู่ (เพิ่ม section / wikilink)
4. ถ้ามี concept ใหม่ที่ยังไม่มี node → สร้าง concept node ใหม่
5. อัพเดต `wiki/index.md` ส่วน Source Summaries
6. อัพเดต `wiki/hotcache.md`

### 2. ตัดต่อภาพ `[ชื่อ PDF]`

**Input**: PDF ใน `raw/`
**Output**: `assets/{folder}/` + `assets/catalog-{name}.md`

ขั้นตอน:
1. ใช้ PyMuPDF (fitz) ตัดรูปจาก PDF ทุกหน้า
2. กรองรูปเล็กกว่า 50px ออก
3. ตั้งชื่อ `p{page:03d}-{idx:02d}.{ext}`
4. บันทึกใน `assets/{ชื่อ PDF}/`
5. สร้าง `assets/catalog-{name}.md` พร้อม context ข้อความจากแต่ละหน้า
6. รายงานจำนวนรูป + ขนาด

### 3. ทำข้อสอบ `[concept node]`

**Input**: ชื่อ concept node หรือ "ทั้งหมด"
**Output**: `wiki/questions/qbank-{topic}.md`

ขั้นตอน:
1. อ่าน concept node target
2. สร้างข้อสอบ 3 ระดับ:
   - **L1 จำ/ระบุ** (5 ข้อ): ถามข้อเท็จจริง ชื่อพืช ส่วนของพืช วงศ์ สรรพคุณ
   - **L2 เชื่อมโยง** (4 ข้อ): เปรียบเทียบพืช เชื่อมโครงสร้างกับการใช้ประโยชน์
   - **L3 วินิจฉัย/ประยุกต์** (3 ข้อ): สถานการณ์จำลอง เลือกพืช/ตำรับ ต้องบูรณาการพฤกษศาสตร์ + ภูมิปัญญา
3. ทุกข้อมี 4 ตัวเลือก + เฉลยพร้อมอธิบาย
4. อัพเดต `wiki/index.md` ส่วน Questions
5. อัพเดต `wiki/assessment/master-assessment.md` ถ้าจำเป็น

### 4. ทำคลังคำศัพท์ `[หมวด]`

**Input**: หมวดคำศัพท์ (เช่น morphology, taxonomy, herbal, taste-element)
**Output**: `wiki/glossary/glossary-{name}.md`

ขั้นตอน:
1. รวบรวมคำศัพท์จาก concept nodes ที่เกี่ยวข้อง
2. สร้างตาราง 3 ภาษา:
   - **ไทย**: คำศัพท์ + ความหมาย
   - **อังกฤษ/ละติน**: ชื่อวิทยาศาสตร์/รากศัพท์ (Latin/Greek etymology)
   - **บาลี/สันสกฤต**: (สำหรับหมวดสมุนไพร/ภูมิปัญญา) + วิธีอ่าน
3. เพิ่ม wikilink เชื่อมกลับ concept nodes
4. อัพเดต `wiki/glossary/glossary-index.md`
5. อัพเดต `wiki/index.md` ส่วน Glossary

### 5. ผลิตข้อสอบ `[qbank files หรือ "ทั้งหมด"]`

**Input**: ไฟล์ qbank ที่ต้องการ
**Output**: `.docx` ข้อสอบ (พร้อมพิมพ์) + `.xlsx` เฉลย

ขั้นตอน:
1. อ่าน qbank files ที่ระบุ
2. สร้าง `.docx`:
   - หน้าปก: ชื่อวิชา วันที่ จำนวนข้อ เวลาสอบ
   - เรียงข้อ L1 → L2 → L3 (หรือสุ่มตามที่ผู้ใช้ต้องการ)
   - แต่ละข้อมี 4 ตัวเลือก ก–ง
   - กระดาษคำตอบท้ายเล่ม
3. สร้าง `.xlsx` เฉลย:
   - คอลัมน์: ข้อที่, คำตอบ, ระดับ (L1/L2/L3), หมวด, คำอธิบาย
   - Sheet สรุปสถิติ: จำนวนข้อต่อระดับ/หมวด
4. บันทึกใน `output/` folder

### 6. checkpoint

**Input**: ไม่มี
**Output**: git commit + อัพเดต hotcache + index

ขั้นตอน:
1. ตรวจ `git status` — ดูไฟล์ที่เปลี่ยน/เพิ่ม
2. อัพเดต `wiki/hotcache.md`:
   - สถานะปัจจุบัน (Phase, เป้าหมาย)
   - สิ่งที่ทำไปใน session นี้
   - สิ่งที่ต้องทำต่อ
3. ตรวจ `wiki/index.md` — เพิ่ม entries ใหม่ถ้ามี
4. `git add` + `git commit` พร้อม message สรุปการเปลี่ยนแปลง
5. รายงานสถานะ git

### 7. ตรวจสอบ

**Input**: ไม่มี
**Output**: รายงานปัญหาที่พบ

ตรวจสอบ:
1. **Broken links**: wikilinks ที่ชี้ไปไฟล์ที่ไม่มี
2. **Orphan nodes**: ไฟล์ที่ไม่มีใครลิงก์ถึง
3. **Missing coverage**:
   - Concept nodes ที่ยังไม่มีข้อสอบ
   - Concept nodes ที่ยังไม่มีคำศัพท์
   - Concept nodes ที่ยังไม่มี source summary
4. **Consistency**: index.md ครบถ้วนไหม, glossary-index ตรงกับไฟล์จริงไหม
5. รายงานเป็นตารางสรุป + แนะนำลำดับความสำคัญในการแก้ไข

### 8. รายงาน

**Input**: ไม่มี (หรือระบุ format: "docx" / "md")
**Output**: เอกสารสรุปภาพรวมโปรเจค

เนื้อหา:
1. **ภาพรวม**: จำนวน concept nodes, sources, questions, glossary terms, images
2. **ความก้าวหน้า**: แต่ละ Layer (S/U/T) ทำไปกี่ %
3. **สถิติข้อสอบ**: จำนวนต่อระดับ (L1/L2/L3) ต่อหมวด
4. **สถิติคำศัพท์**: จำนวนต่อหมวด
5. **ขนาดข้อมูล**: vault size, image count, PDF count
6. **TODO**: สิ่งที่ยังไม่ได้ทำ เรียงลำดับความสำคัญ
7. บันทึกใน `output/report-{date}.md` (หรือ `.docx`)

### 9. วิเคราะห์เมนู `[ชื่อเมนู หรือ id หรือ รอบ 1-5]`

**Input**: เมนูใน `data/menu-db.json` หรือชุดรอบ (แกง/ต้ม · ยำ · อีสาน · นึ่งย่าง · ของว่าง)
**Output**: Tier 2 ใน `data/menu-analysis-tier2-*.json` + (ถ้าระบุ `ลึก`) `wiki/menus/{id}.md`

ขั้นตอน:
1. อ่าน entry เดิม + แหล่งอ้างอิง (gj.mahidol, cra.ac.th, prijnr ฯลฯ)
2. เติม **Tier 2**: `ingredients{core,optional}`, `layerS[]`, `whenCooked{suitAudience,avoidFor,summary}`, `analysisTier: 2`
3. รัน `python scripts/merge-menu-analysis.py` รวมเข้า `menu-db.json`
4. เมนูสำคัญ → **Tier 3** wiki ตามแม่แบบ [[food-analysis-ttm]] + แกงหน่อไม้
5. อัพเดต `wiki/food-recommender.md` · `wiki/reference-sources.md`

## หลักการทั่วไป

- อ่าน `wiki/hotcache.md` ก่อนเสมอเมื่อเริ่ม session ใหม่
- ภาษาหลักของเนื้อหาคือ **ไทย** ยกเว้นศัพท์เทคนิค/ชื่อวิทยาศาสตร์
- ชื่อวิทยาศาสตร์ของพืชเขียนแบบ *Genus species* (italic) พร้อมวงศ์ (Family)
- ทุกไฟล์มี YAML frontmatter (title, type, created, tags)
- ใช้ Obsidian wikilink syntax `[[name]]` ไม่ใช้ markdown link
- รูปภาพใช้ `![[assets/folder/image.png]]` — ไฟล์อยู่ใน `assets/` (ไม่ใช่ root vault)
- commit message เป็นภาษาอังกฤษ
