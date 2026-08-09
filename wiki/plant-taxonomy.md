---
title: "การจำแนกพืช"
id: S8
type: concept
layer: structure
created: 2026-08-09
updated: 2026-08-09
prerequisites:
  - plant-morphology
source_type: external
references:
  - "https://botany.dnp.go.th/"
  - "https://botany.dnp.go.th/mplant/about.html"
  - "https://bdn-thp.dmsc.moph.go.th/home"
  - "https://bdn.dmsc.moph.go.th/th/detailAll/1080/ebook/136"
  - "https://il.mahidol.ac.th/e-media/plants/webcontent3/main.html"
  - "https://sireebioportal.mahidol.ac.th/plant/"
  - "https://www.scimath.org/project/item/6355-zingiberaceae"
tags:
  - structure
  - taxonomy
---

# การจำแนกพืช (Plant Taxonomy)

> 🌐 **source_type: external** — scaffold จากแหล่งเปิด (กรมอุทยานฯ · THP · มหิดล · สสวท.) · เล่มสมพร ภูติยานันต์ อยู่ใน Textbook queue ของ [[reference-sources]] (ยังไม่ย่อย) · ศัพท์ → [[glossary/glossary-taxonomy]]

## สาระสำคัญ

**การจำแนกพืช (taxonomy / systematics)** คือการจัดกลุ่มพืชตามความสัมพันธ์ และตั้งชื่ออย่างเป็นระบบ เพื่อให้ระบุได้ว่าเป็นพืชชนิดใด — ในโปรเจกต์นี้เป็นสะพาน **Naming → Identifying** ของ Botany Literacy และเป็นขั้นแรกของกรอบวิเคราะห์สมุนไพรใน [[thai-herbal]]

```
สัณฐาน ([[plant-morphology]])  →  จำแนก/ระบุ (node นี้)  →  ใช้เป็นยา/อาหาร ([[thai-herbal]] · Layer U)
```

## 1. อันดับอนุกรมที่ใช้บ่อย (Taxonomic ranks)

โฟกัสระดับที่ต้องใช้จริงตอนเรียกชื่อสมุนไพร/พืชอาหาร:

| อันดับ (ไทย) | อังกฤษ | ตัวอย่าง |
|--------------|--------|----------|
| อาณาจักร | Kingdom | Plantae |
| ไฟลัม/หมวด | Phylum / Division | Magnoliophyta (พืชดอก) |
| ชั้น | Class | — |
| อันดับ | Order | Zingiberales |
| **วงศ์** | **Family** | Zingiberaceae (วงศ์ขิง) |
| **สกุล** | **Genus** | *Zingiber* |
| **ชนิด** | **Species** | *Zingiber officinale* (ขิง) |

> ในงาน ethnobotany / สมุนไพรไทย มักเขียน **ชื่อวิทยาศาสตร์ + วงศ์** เสมอ

## 2. การตั้งชื่อทวินาม (Binomial nomenclature)

รูปแบบมาตรฐาน: ***Genus species*** Author  
ตัวอย่าง: *Zingiber officinale* Roscoe · วงศ์ Zingiberaceae

กฎที่ใช้ใน vault:

1. ชื่อสกุลขึ้นต้นด้วยตัวพิมพ์ใหญ่ · ชื่อชนิดตัวพิมพ์เล็กทั้งคำ
2. เขียน italic ทั้งสองส่วน (ใน Markdown ใช้ `*...*`)
3. ชื่อผู้ตั้งชื่อ (author citation) ใส่ได้เมื่ออ้างอิงเชิงวิชาการ — ใน concept node ทั่วไปอาจละได้ถ้าไม่จำเป็น
4. **ชื่อไทย ≠ ชื่อวิทยาศาสตร์** — ชื่อพื้นเมืองซ้ำ/ทับกันได้ ต้องยึดชื่อวิทยาศาสตร์ + ตรวจกับฐานชื่อไทยทางการ

## 3. แหล่งยืนยันชื่อในประเทศไทย

| บทบาท | แหล่ง | ใช้เมื่อ |
|--------|-------|---------|
| ชื่อไทย ↔ วิทยาศาสตร์ | [ชื่อพรรณไม้แห่งประเทศไทย / DNP](https://botany.dnp.go.th/) (เต็ม สมิตินันทน์ · ฐานออนไลน์) | Naming · แก้ชื่อเรียกท้องถิ่น |
| ค้นวงศ์/ชนิด | [Siree bioportal — มหิดล](https://sireebioportal.mahidol.ac.th/plant/) | ตรวจ Family / scientific name |
| ฝึกระบุจากลักษณะ | [รูปวิธานระบุพืช — มหิดล](https://il.mahidol.ac.th/e-media/plants/webcontent3/main.html) | Identifying (ต้องมีพื้นฐานสัณฐาน) |

## 4. จากสัณฐาน → ระบุชนิด (Identifying)

ใช้ลักษณะจาก Layer S เป็น “กุญแจ”:

| ลักษณะ | node | ใช้แยกอะไรบ่อย |
|--------|------|----------------|
| วิสัย (ไม้ต้น/พุ่ม/เลื้อย/ล้มลุก) | [[plant-morphology]] | กรองกลุ่มใหญ่ |
| ใบ (เดี่ยว/ประกอบ, ขอบ, เส้นใบ) | [[leaf-morphology]] | สกุล–ชนิดใกล้เคียง |
| ดอก (สมมาตร, วงกลีบ, ช่อ) | [[flower-morphology]] | วงศ์เด่น |
| ผล/เมล็ด | [[fruit-seed-morphology]] | ยืนยันชนิด |
| เนื้อเยื่อ/ภาคตัด (ขั้นสูง) | [[plant-tissue]] | ตรวจ crude drug (THP) |

**รูปวิธาน (identification key)** = ชุดคำถามเลือกได้ทีละคู่ (เช่น ใบเลี้ยงคู่ vs ใบเลี้ยงเดี่ยว) จนเหลือชนิด — ฝึกได้จากสื่อมหิดลด้านบน

## 5. ตรวจเอกลักษณ์สมุนไพร (เชื่อม Layer T) 🌐

ตำรามาตรฐานยาสมุนไพรไทย (**THP** — กรมวิทยาศาสตร์การแพทย์) กำหนดวิธียืนยันวัตถุดิบยาสมุนไพร:

1. **Description of the plant** — ลักษณะพฤกษศาสตร์ของต้นสด
2. **Macroscopical** — ดูด้วยตา/แว่นขยาย (รูปร่าง สี เนื้อ crude drug)
3. **Microscopical** — ภาคตัด/ผงยา (เซลล์ ผลึก เส้นใย ฯลฯ)
4. **Identification อื่น** — เช่น TLC ในหลาย monograph

→ กรอบ [[thai-herbal]] ข้อ 1 “ระบุพืช” ขยายเป็น: **ชื่อถูกต้อง (DNP) + ลักษณะตรง monograph (THP)**

ที่มา: [THP portal](https://bdn-thp.dmsc.moph.go.th/home) · [THP 2021 Volume II (PDF)](https://bdn.dmsc.moph.go.th/th/detailAll/1080/ebook/136)

## 6. ตัวอย่างวงศ์สมุนไพรไทย (Zingiberaceae)

งานสำรวจอนุกรมวิธานวงศ์ขิง (SciMath / มข.) แสดงรูปแบบที่ใช้จริง: **นับสกุล–ชนิด · สร้างรูปวิธาน · บรรยายลักษณะ + นิเวศ**

สกุลที่พบบ่อยในสมุนไพร/อาหารไทย:

| สกุล | ตัวอย่างชนิด (แนว) | ส่วนใช้บ่อย |
|------|---------------------|-------------|
| *Zingiber* | ขิง | เหง้า → [[stem-morphology]] |
| *Curcuma* | ขมิ้นชัน | เหง้า |
| *Kaempferia* | เปราะหอม / ว่าน | เหง้า |
| *Boesenbergia* | กระชาย | เหง้า/ราก |
| *Alpinia* | ข่า | เหง้า |

ที่มาตัวอย่างวงศ์: [พืชวงศ์ขิงในอุทยานแห่งชาติภูเวียง — SciMath](https://www.scimath.org/project/item/6355-zingiberaceae)

## Misconceptions ที่พบบ่อย

- คิดว่าชื่อไทยอย่างเดียวพอระบุชนิด → ชื่อพื้นเมืองซ้ำข้ามพืชได้ ต้องมีชื่อวิทยาศาสตร์
- สับสน **สกุล** กับ **ชนิด** → สกุลคือกลุ่ม (*Zingiber*) ชนิดคือหน่วย (*Z. officinale*)
- คิดว่า taxonomy = ท่องชื่ออย่างเดียว → ต้องผูกกับสัณฐานและ (ถ้าเป็นยา) มาตรฐานเอกลักษณ์

## Prerequisites

- [[plant-morphology]] (และอวัยวะย่อยตามที่ระบุ)

## Leads to

- [[glossary/glossary-taxonomy]] — คลังศัพท์อนุกรมวิธาน
- [[thai-herbal]] — ใช้การระบุพืชเป็นขั้นแรกของกรอบสมุนไพร
- [[flower-morphology]] — ตารางลักษณะดอกเด่นรายวงศ์
- (คิว) เล่มสมพร ภูติยานันต์ — Textbook queue ใน [[reference-sources]]

## ที่มา (External sources) — เข้าถึง 2026-08-09

- [พฤกษศาสตร์ป่าไม้ / ชื่อพรรณไม้แห่งประเทศไทย — กรมอุทยานฯ](https://botany.dnp.go.th/)
- [Thai Plant Names database — DNP](https://botany.dnp.go.th/mplant/about.html)
- [ตำรามาตรฐานยาสมุนไพรไทย (THP)](https://bdn-thp.dmsc.moph.go.th/home)
- [Thai Herbal Pharmacopoeia 2021 Volume II](https://bdn.dmsc.moph.go.th/th/detailAll/1080/ebook/136)
- [รูปวิธานอินเตอร์แอคทีฟระบุพืช — มหิดล](https://il.mahidol.ac.th/e-media/plants/webcontent3/main.html)
- [ฐานข้อมูลทรัพยากรชีวภาพ (plant) — มหิดล](https://sireebioportal.mahidol.ac.th/plant/)
- [พืชวงศ์ขิง (Zingiberaceae) — SciMath](https://www.scimath.org/project/item/6355-zingiberaceae)
