---
title: "Hotcache"
type: meta
created: 2026-07-03
updated: 2026-07-03
tags:
  - meta
---

# Hotcache

## สถานะปัจจุบัน

- **Phase**: Layer S ครบ 5 อวัยวะ + Layer T ครบ (hub/รสยา/ธาตุ/ตำรับ) + glossary morphology
- **เป้าหมายถัดไป**: รอข้อมูลเพิ่ม (ผู้ใช้แจ้ง) ก่อนทำข้อสอบ · ระหว่างรอ: glossary-herbal (บาลี), เนื้อเยื่อ/taxonomy
- **ระบบอ้างอิง**: มี [[reference-sources]] แยก certified ✅ / external 🌐 — external ต้องมี URL เสมอ
- **ตัดสินใจแล้ว**:
  - โครงสร้าง + workflow เหมือน body-xambrain (ดู CLAUDE.md)
  - Framework 3 Layer: S=พฤกษศาสตร์ / U=การใช้ประโยชน์ / T=ภูมิปัญญาไทย
  - Botany Literacy 5 ระดับ: Naming → Identifying → Using → Connecting → Conserving
  - ข้อสอบ 3 ระดับ: L1 จำ/ระบุ, L2 เชื่อมโยง, L3 วินิจฉัย/ประยุกต์
  - Git local only

## Last Session (2026-07-03)

- Scaffold โปรเจกต์ botany-xambrain + CLAUDE.md/overview/index/hotcache + git init
- **ย่อยเนื้อหา สัณฐานวิทยาพืช** (3 PDF: ราก 29น. + ลำต้น 26น. text-based, ใบ 4น. image)
- สร้าง [[source-plant-morphology]] + concept nodes: [[plant-morphology]] (hub), [[root-morphology]], [[stem-morphology]], [[leaf-morphology]] (provisional)
- ใส่ cross-layer link: เหง้าขิง/ข่า/ขมิ้น = ลำต้นใต้ดิน → สมุนไพร (Layer T)

## สิ่งที่ทำแล้ว

1. ~~Scaffold โปรเจกต์ + framework~~ ✅
2. ~~ย่อยเนื้อหา สัณฐานวิทยา (ราก/ลำต้น/ใบ)~~ ✅
3. ~~ตัดต่อภาพใบ (8 รูป) + ถอดเนื้อหาเข้า [[leaf-morphology]]~~ ✅
4. ~~วางระบบอ้างอิง [[reference-sources]] + node ดอก/ผล-เมล็ด (external, มี URL)~~ ✅
5. ~~เริ่ม Layer T: [[thai-herbal]] hub + [[herbal-taste-9]] (external, อ้างอิง ม.อ.)~~ ✅
6. ~~[[dhatu-4-plants]]: ธาตุ 4 + รสแก้ธาตุ + เบญจกูล เชื่อม Layer S ครบวง~~ ✅
7. ~~[[herbal-formula]] ตำรับ ตรีกฏุก/ตรีผลา/เบญจกูล (external)~~ ✅
8. ~~glossary-morphology (ไทย/อังกฤษ/รากศัพท์) + glossary-index~~ ✅

## ต้องทำต่อ

1. **ข้อสอบ 3 ระดับ — พักไว้ก่อน (ผู้ใช้ขอรอข้อมูลเยอะกว่านี้)**
2. glossary-herbal (รสยา/ธาตุ + บาลี/สันสกฤต)
3. ขยาย Layer S: เนื้อเยื่อพืช, taxonomy — หา certified source ก่อนถ้าได้
3. เริ่ม Layer T: สมุนไพรไทย, รสยา 9 รส, เชื่อมธาตุ 4
4. ทำข้อสอบ + คลังคำศัพท์ (morphology terms 3 ภาษา)
5. (ระยะยาว) เว็บ interactive คล้ายแผนของ body-xambrain

## Open Questions

- เนื้อหาตั้งต้นจะมาจากแหล่งไหน? (ตำราเรียน / slide อาจารย์ / งานภาคสนาม)
- ขอบเขตพืช: เน้นสมุนไพร หรือครอบคลุมพืชอาหาร+เครื่องใช้+ย้อมสีด้วย?
