---
title: "แหล่งอ้างอิงและระดับการรับรอง"
type: reference
created: 2026-07-03
updated: 2026-08-09
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

## Textbook queue (ตำราที่สนใจ — ยังไม่ย่อย)

ตำราวิชาการที่บันทึกไว้สำหรับย่อยภายหลัง · ระดับ **external** · ต้องมี URL เข้าถึง

| วันที่ | หนังสือ | ผู้แต่ง / สำนักพิมพ์ | เข้าถึง | เป้าหมาย |
|--------|---------|----------------------|--------|----------|
| 2026-08-09 | *การตรวจเอกลักษณ์พืชสมุนไพร : พฤกษอนุกรมวิธาน* · ISBN `9789740337584` · 867 หน้า · PDF | รศ.สมพร ภูติยานันต์ · สำนักพิมพ์จุฬาลงกรณ์มหาวิทยาลัย | [ARU Hibrary (ebook)](https://elibrary-aru.hibrary.me/rent/ebook/detail/f7344b52-4529-45c3-b745-46a466ee7fb5) · ม.ราชภัฏพระนครศรีอยุธยา · Rent (คิว 1 · ยืม 5 วัน) · สมัครด้วย `@aru.ac.th` | อัปเกรด taxonomy เมื่อเข้าถึงได้ · คู่ขนานกับแหล่งเปิดด้านล่าง |

## External source log (แหล่งอินเทอร์เน็ตที่ใช้แล้ว)

| วันที่ | หัวข้อ | แหล่ง | ใช้ใน node |
|--------|--------|-------|-----------|
| 2026-08-20 | **ตรวจสอบลักษณะใบกัญชา**: ใบประกอบนิ้วมือ 3–9(–11) ใบย่อย · ขนาดใบย่อย · หูใบรูปแถบ · ก้านใบ 2–7 ซม. · ใบล่างตรงข้าม-ใบบนสลับ · จำนวนใบย่อยลดลงเมื่อใกล้ช่อดอก | [Flora of China](http://www.efloras.org/florataxon.aspx?flora_id=2&taxon_id=200006342) · [Flora of North America (E. Small) ผ่าน OregonFlora](https://oregonflora.org/taxa/index.php?taxon=3580) · [Illinois Wildflowers](https://www.illinoiswildflowers.info/weeds/plants/hemp.htm) · เทียบกับ [อพ.สธ.](https://www.rspg.or.th/plants_data/plantdat/cannabid/csativ_1.htm) · [QSBG](http://www.qsbg.org/Database/plantdb/mdp/medicinal-specimen.asp?id=452) | [[plant-cannabis]] §5 · [[plant-morphology-comparison]] |
| 2026-08-20 | สัณฐานวิทยากัญชา: habit · ใบประกอบนิ้วมือ · dioecious · achene · ไตรโครม 3 แบบ · sativa/indica/ruderalis · กัญชา vs กัญชง | [อพ.สธ. rspg](https://www.rspg.or.th/plants_data/plantdat/cannabid/csativ_1.htm) · [QSBG](http://www.qsbg.org/Database/plantdb/mdp/medicinal-specimen.asp?id=452) · [ม.มหิดล Simple Science](https://science.mahidol.ac.th/simple-science/2022/04/21/know-about-thai-cannabis/) · [PETROMAT](https://petromat.org/home/cannabis/) · [PMC10485653](https://pmc.ncbi.nlm.nih.gov/articles/PMC10485653/) · [PMC10071647](https://pmc.ncbi.nlm.nih.gov/articles/PMC10071647/) | [[plant-cannabis]] |
| 2026-08-20 | สถานะกฎหมายกัญชา: ประกาศ สธ. สมุนไพรควบคุม (กัญชา) พ.ศ. 2568 (ช่อดอก) · 16 ตำรับยาแผนไทยที่มีกัญชาปรุงผสม | [กองกัญชาทางการแพทย์ dtam](https://med-cannabis.dtam.moph.go.th/law/2418/) · [กรมประชาสัมพันธ์](https://www.prd.go.th/th/content/category/detail/id/35/iid/413114) · [กรมการแพทย์ dms](https://www.dms.go.th/backend/Content/Content_File/Publication/Attach/25651122182642PM_Guidance%20Updated%20V5_update%2029092022_edited2.pdf) · [MGR Online 16 ตำรับ](https://mgronline.com/qol/detail/9620000023197) · [TheCoverage](https://www.thecoverage.info/news/content/2539) | [[plant-cannabis]] §11–12 |
| 2026-08-20 | อัญชัน *Clitoria ternatea*: ใบประกอบขนนกปลายคี่ · ดอกรูปดอกถั่ว resupinate · ฝัก · ternatin/anthocyanin · ข้อควรระวังกับยาต้านเกล็ดเลือด | [QSBG](http://www.qsbg.org/Database/plantdb/mdp/medicinal-specimen.asp?id=574) · [data.addrun.org](https://data.addrun.org/plant/archives/200-clitoria-ternatea-l) · [ม.มหิดล เภสัช](https://pharmacy.mahidol.ac.th/th/knowledge/article/226/) · [มทส.](https://crspg.sut.ac.th/plant/living_plants_detail.php?id=213) | [[plant-anchan]] |
| 2026-08-20 | เถาวัลย์เปรียง *Derris scandens*: ไม้เถาเนื้อแข็ง · ใบประกอบขนนก 7–9 ใบย่อย · ช่อกระจะ 20–30 ซม. · เครื่องยาจากเถา · บัญชียาหลัก · หลักฐานเทียบ diclofenac/ibuprofen | [ฐานข้อมูลเครื่องยา ม.อุบล](https://phar.ubu.ac.th/herb-DetailThaicrudedrug/63) · [สสจ.ปัตตานี](https://sites.google.com/view/ttmpattani/) · [ม.ขอนแก่น](https://pharmoffice.kku.ac.th/iss/smhcontrol/herb/330) · [ม.มหิดล medplant](https://medplant.mahidol.ac.th/user/reply.asp?id=7739) | [[plant-thaowan-priang]] |
| 2026-08-20 | หม่อน *Morus alba*: ยางขาว · ใบ trinerved และหลายรูป · ช่อหางกระรอก · ผลรวม sorosis · ใบรสจืดเย็น | [ฐานข้อมูลสมุนไพร ม.อุบล](https://phar.ubu.ac.th/herb-DetailPhargarden/125) · [กรมหม่อนไหม qsds](https://qsds.go.th/newosrd/) · [ARDA agknowledge](http://agknowledge.arda.or.th/silk&sericulture/?page_id=694) · [World Flora Online](https://www.worldfloraonline.org/taxon/wfo-0000447905) | [[plant-mon]] |
| 2026-08-20 | ตะแบกนา *L. floribunda* + อินทนิลบก *L. macrocarpa*: เปลือกเป็นลักษณะวินิจฉัย · ขนาดใบ/ดอก/ผล · แยกตะแบก-เสลา-อินทนิลน้ำ-อินทนิลบก | [data.addrun.org ตะแบกนา](https://data.addrun.org/plant/archives/d2051-lagerstroemia-floribunda-jack) · [กรมป่าไม้ forprod](https://forprod.forest.go.th/forprod/techtransfer/document/) · [ม.อ.ปัตตานี](https://building.pn.psu.ac.th/psupn_tree/?page=detail-plants&id=73) · [ราชบัณฑิตยสภา](http://legacy.orst.go.th/) · [อุทยานหลวงราชพฤกษ์](https://rptree.royalparkrajapruek.org/) | [[plant-tabaek]] · [[plant-inthanin-bok]] |
| 2026-08-22 | เขาแกะ *Rhynchostylis coelestis*: กล้วยไม้อิงอาศัย monopodial · รากอากาศ/velamen · ใบ distichous conduplicate ปลายเว้า 2 แฉก · ช่อกระจะตั้งขึ้น 20–50 ดอก · กลีบปาก+เดือย+เส้าเกสร · แยกจากช้างกระ/ไอยเรศ | [วิกิพีเดียไทย](https://th.wikipedia.org/wiki/%E0%B9%80%E0%B8%82%E0%B8%B2%E0%B9%81%E0%B8%81%E0%B8%B0) · [หมอชาวบ้าน](https://www.doctor.or.th/article/detail/10245) · [อพ.สธ. ม.อุบลฯ](https://rspgubu.ubu.ac.th/details.php?list=3&id=5) · [NECTEC LST](https://lst.nectec.or.th/encyclopedia/wikipedia/plant_taxonomy/view/index.php?namelink=%E0%B9%80%E0%B8%82%E0%B8%B2%E0%B9%81%E0%B8%81%E0%B8%B0&cid=25) · [Wikipedia EN](https://en.wikipedia.org/wiki/Rhynchostylis_coelestis) · [Monaco Nature Encyclopedia](https://www.monaconatureencyclopedia.com/rhynchostylis-coelestis-2/?lang=en) · [OrchidSpecies](https://www.orchidspecies.com/rhyncoelestis.htm) · [e-Monocot Aeridinae](http://aeridinae.e-monocot.org/taxonomy/term/9226/descriptions) | [[plant-khao-kae]] |
| 2026-08-23 | แคบ้าน *Sesbania grandiflora*: ใบขนนกปลายคู่ 10–30 คู่ · ช่อกระจะ 2–4 ดอก · ดอกถั่ว 5–10 ซม. · **เกสรเพศผู้ 10 อัน (9)+1** · ฝักแก่ 30–50 ซม. · ปมรากตรึงไนโตรเจน · สรรพคุณดอก/ใบ/เปลือก · **ตรวจพบข้อมูลไทยที่ขัดแย้ง: "เกสร 60 อัน" · "ฝัก 8–15 ซม."** | [วิกิพีเดียไทย](https://th.wikipedia.org/wiki/%E0%B9%81%E0%B8%84) · [QSBG พฤกษศาสตร์พื้นบ้าน](http://www.qsbg.org/Database/plantdb/mdp/medicinal-specimen.asp?id=805) · [BGO Plant Database](http://www.qsbg.org/database/botanic_book%20full%20option/search_detail.asp?botanic_id=797) · [MedThai](https://medthai.com/%E0%B8%94%E0%B8%AD%E0%B8%81%E0%B9%81%E0%B8%84/) · [JIRCAS Thai vegetables](https://www.jircas.go.jp/en/database/thaivege/091) · [Monaco Nature Encyclopedia](https://www.monaconatureencyclopedia.com/sesbania-grandiflora-2/?lang=en) · [Wikipedia EN](https://en.wikipedia.org/wiki/Sesbania_grandiflora) · [NTBG](https://ntbg.org/database/plants/detail/sesbania-grandiflora) · [อพ.สธ. ม.ราชภัฏบุรีรัมย์](https://rspgdb.bru.ac.th/biologically/views/plants/show.php?id=33) | [[plant-khae-ban]] |
| 2026-09-15 | คูน/ราชพฤกษ์ *Cassia fistula*: ไม้ต้น · **วงศ์ย่อย Caesalpinioideae** ดอกเหลืองแผ่กาง (ไม่ใช่ดอกถั่ว) · เกสร 10 อันขนาดไม่เท่ากัน · ฝักทรงกระบอกยาว **ไม่แตก** · เนื้อในฝักเป็นยาระบาย · ไม้ประจำชาติไทย | ใบงานกลุ่มต้นคูน (`Downloads/สันฐาน/ต้นคูณ.pdf`) · [QSBG BGO Plant Database](http://www.qsbg.org/database/botanic_book%20full%20option/search_detail.asp) · [สำนักงานหอพรรณไม้ DNP](https://www.dnp.go.th/botany/) · [ม.มหิดล เภสัช](https://pharmacy.mahidol.ac.th/th/knowledge/) | [[plant-khun]] |
| 2026-09-15 | ส้มลม *Aganonerion polymorphum* (Apocynaceae): ไม้เถา **น้ำยางขาว** · ใบเดี่ยวเรียงตรงข้าม รูปหลายแบบ · กลีบดอกเชื่อมเป็นหลอด สีชมพู · รังไข่ 2 คาร์เพลแยก → **ผล follicle เป็นคู่ แตก** · เมล็ดมีปุยขาวปลิวลม · ยอด/ใบรสเปรี้ยวเป็นผัก (lá giang) | ใบงานกลุ่มต้นคูน–ส่วนส้มลม (`Downloads/สันฐาน/ต้นส้มลม-ณิชาภา ภู่จีน .pdf`) · [QSBG BGO Plant Database](http://www.qsbg.org/database/botanic_book%20full%20option/search_detail.asp) · [World Flora Online](https://www.worldfloraonline.org/) · [อพ.สธ. rspg ผักพื้นบ้าน](https://www.rspg.or.th/plants_data/) | [[plant-somlom]] |
| 2026-09-15 | น้ำใจใคร่ *Olax psittacorum* (Olacaceae): ไม้พุ่มรอเลื้อย 2–5 ม. · ไม่มียาง · ใบเดี่ยวเรียงสลับ หนาเป็นมัน · ดอกขาวเล็ก **กลีบดอก 3 · เกสรผู้ 3 · ยอดเกสร 3 แฉก · กลีบเลี้ยง 5 รูปถ้วย** · ผลกลมเมล็ดเดียว · ชื่อท้องถิ่นหลายภาค (นางจุม/กระทกรก/ลูกไข่แลน) | ใบงานน้ำใจใคร่ (`Downloads/สันฐาน/น้ำใจใคร่.pdf`) · [QSBG BGO Plant Database](http://www.qsbg.org/database/botanic_book%20full%20option/search_detail.asp) · [World Flora Online](https://www.worldfloraonline.org/) · [อพ.สธ. rspg](https://www.rspg.or.th/plants_data/) | [[plant-namjaikhrai]] |
| 2026-09-15 | มะเดื่อชุมพร/อุทุมพร *Ficus racemosa* (Moraceae): ไม้ต้น **น้ำยางขาว** · ใบเดี่ยวเรียงสลับ มีหูใบ (สกุล Ficus) · ช่อดอก **syconium (ฐานรองดอกปิด ดอกซ่อนภายใน)** · **cauliflory** ออกตามลำต้น/กิ่งแก่ · ผสมเกสรด้วย **แตนมะเดื่อ (fig wasp)** · ผลรวม fig สุกแดงม่วง · อุทุมพรในคติพุทธ (สำนวน "เห็นดอกมะเดื่อ") · คู่เทียบ syconium ↔ sorosis (หม่อน) | ใบงานมะเดื่อชุมพร–มยุรี ผิวลมูล (`Downloads/สันฐาน/มะเดื่อชุมพร -มยุรี ผิวลมูล.pdf`) · [QSBG BGO Plant Database](http://www.qsbg.org/database/botanic_book%20full%20option/search_detail.asp) · [World Flora Online](https://www.worldfloraonline.org/) · [ม.อุบล ฐานข้อมูลสมุนไพร](https://phar.ubu.ac.th/herb-thaicrudedrug/) | [[plant-maduea-chumphon]] |
| 2026-09-15 | สุพรรณิการ์/ฝ้ายคำ *Cochlospermum regium* (Bixaceae, Malvales): ไม้ต้นผลัดใบ · ใบเดี่ยวแฉกฝ่ามือ 5 แฉก เส้นใบฝ่ามือ · ดอกเหลือง 5 กลีบ actinomorphic เกสรจำนวนมาก **อับเรณูเปิดรู (poricidal/buzz)** · แคปซูลแตก 3–5 ซีก · เมล็ดรูปไตมีปุยฝ้าย (floss) กระจายโดยลม · ⚠️ ไทยมักใช้ *C. religiosum* | ใบงานกลุ่ม–สุพรรณิการ์ (`Downloads/สันฐาน/สุพรรณิการ์และจั๋งงานกลุ่ม.pdf`) · [QSBG BGO](http://www.qsbg.org/database/botanic_book%20full%20option/search_detail.asp) · [World Flora Online](https://www.worldfloraonline.org/) · [อพ.สธ. rspg](https://www.rspg.or.th/plants_data/) | [[plant-supphannika]] |
| 2026-09-15 | จั๋ง *Rhapis subtilis* (Arecaceae, monocot): ปาล์มแตกกอเล็ก ลำเรียวคล้ายไผ่มีเส้นใยสีน้ำตาลหุ้ม · รากฝอย · ใบประกอบรูปพัด เส้นใบขนาน · ดอกแยกเพศวง 3 (กลีบเลี้ยง 3/กลีบดอก 3/เกสร 6/คาร์เพล 3) · ผล berry กลมสีขาว 1 เมล็ด · **monocot ตัวที่ 2 (เทียบ dicot สุพรรณิการ์)** | ใบงานกลุ่ม–จั๋ง (`Downloads/สันฐาน/สุพรรณิการ์และจั๋งงานกลุ่ม.pdf`) · [QSBG BGO](http://www.qsbg.org/database/botanic_book%20full%20option/search_detail.asp) · [World Flora Online](https://www.worldfloraonline.org/) · [อพ.สธ. rspg](https://www.rspg.or.th/plants_data/) | [[plant-chang]] |
| 2026-09-15 | ประดู่ป่า *Pterocarpus macrocarpus* (Fabaceae/Faboideae): ไม้ต้นใหญ่ผลัดใบ · เปลือกมี **ยางสีแดง (kino)** · ใบประกอบขนนกปลายคี่ · ดอกถั่วสีเหลือง เกสร (9)+1 · **ผลฝักกลมแบนมีปีกโดยรอบ ไม่แตก กระจายด้วยลม (samaroid legume)** · ไม้เศรษฐกิจ · ดอกสัญลักษณ์กองทัพเรือ · แยกจากประดู่บ้าน *P. indicus* | ตารางคำตอบต้นประดู่ป่า (`Downloads/สันฐาน/ตารางคำตอบต้นประดู่ป่า_แบ่งตามกลุ่ม.xlsx`) · [QSBG BGO](http://www.qsbg.org/database/botanic_book%20full%20option/search_detail.asp) · [หอพรรณไม้ DNP](https://www.dnp.go.th/botany/) · [World Flora Online](https://www.worldfloraonline.org/) | [[plant-pradu-pa]] |
| 2026-08-09 | การจำแนกพืช (taxonomy): อันดับอนุกรม · binomial · ชื่อไทย–วิทยาศาสตร์ · ตรวจเอกลักษณ์สมุนไพร | [ชื่อพรรณไม้แห่งประเทศไทย / DNP](https://botany.dnp.go.th/) · [Thai Plant Names](https://botany.dnp.go.th/mplant/about.html) · [THP กรมวิทย์ฯ](https://bdn-thp.dmsc.moph.go.th/home) · [THP 2021 Vol.II](https://bdn.dmsc.moph.go.th/th/detailAll/1080/ebook/136) · [รูปวิธานระบุพืช มหิดล](https://il.mahidol.ac.th/e-media/plants/webcontent3/main.html) · [Siree bioportal มหิดล](https://sireebioportal.mahidol.ac.th/plant/) · [SciMath วงศ์ขิง](https://www.scimath.org/project/item/6355-zingiberaceae) | [[plant-taxonomy]] · [[glossary/glossary-taxonomy]] · [[thai-herbal]] |
| 2026-08-09 | เคส THP 5 ชนิด: ขิง · ขมิ้นชัน · กระชาย · ฟ้าทะลายโจร · มะขามป้อม (macro/micro + ส่วน crude drug) | [KHING](https://bdn-thp.dmsc.moph.go.th/ebook/qQWcZ3tkpR9gC3q0GT5gMJq0qT5co3uw) · [KHAMIN CHAN](https://bdn-thp.dmsc.moph.go.th/ebook/qQycBUtlpR9gC3q0GT5gMJq0qT5co3uw) · [KRACHAI](https://bdn-thp.dmsc.moph.go.th/ebook/qQMcAUtlpR9gC3q0GT5gMJq0qT5co3uw) · [FA THALAI](https://bdn-thp.dmsc.moph.go.th/ebook/nGu4A3OCoG93qRkhoJIaqUEhnJ94LjWewEb3QWewEb3Q) · [MAKHAM POM](https://bdn-thp.dmsc.moph.go.th/ebook/qQEcZUtkpR9gC3q0GT5gMJq0qT5co3uw) | [[plant-taxonomy]] §7 · [[glossary/glossary-taxonomy]] · [[thai-herbal]] |
| 2026-08-09 | glossary-herbal: รสยา 9+1 · ธาตุ 4 · พิกัด (บาลี/สันสกฤต + วิธีอ่าน) | [ม.อ. รสยา](https://www.ttmed.psu.ac.th/th/blog/86) · [Ayurvedic Assoc. TH — รสยา](http://ayurvedicassociationofthailand.blogspot.com/2014/04/9.html) · [มหิดล ธาตุ](https://pharmacy.mahidol.ac.th/th/knowledge/article/98/) | [[glossary/glossary-herbal]] · [[herbal-taste-9]] · [[dhatu-4-plants]] · [[herbal-formula]] |
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
