# สถาปัตยกรรมระบบ — การทำสมองที่สอง (Xambrain)

> เอกสารออกแบบสถาปัตยกรรมของทั้ง ecosystem
> อ่านคู่กับ [`README.md`](README.md) (ภาพรวม) และ [`CLAUDE.md`](CLAUDE.md) (สเปกทางเทคนิค + workflow)

---

## 1. หลักการออกแบบ (Design Principles)

| # | หลักการ | ความหมายเชิงปฏิบัติ |
|---|---------|---------------------|
| 1 | **Local-first** | ความรู้และข้อมูลส่วนตัวอยู่ในเครื่องผู้ใช้ก่อน — ไม่บังคับพึ่ง server |
| 2 | **Plain text เป็นแหล่งความจริง** | เนื้อหา = Markdown, ข้อมูล = JSON — เปิดได้ทุกที่ ไม่ผูก vendor |
| 3 | **AI เป็นผู้ช่วย ไม่ใช่ผู้ตัดสิน** | กฎ (rules) เลือกเมนู/ตรวจข้อสอบ · LLM แค่ช่วยอธิบาย/ร่าง |
| 4 | **แยกชั้น (separation of concerns)** | ความรู้ · วัดผล · ใช้จริง แยกกัน เปลี่ยนทีละส่วนได้ |
| 5 | **ครูไทยต้องใช้ได้** | ติดตั้งไฟล์เดียว ดับเบิลคลิก ไม่มี Git/Terminal |
| 6 | **ส่งเสริมสุขภาพ ไม่วินิจฉัย** | ทุกจุดที่ให้คำแนะนำสุขภาพต้องมีคำเตือน |
| 7 | **Deterministic ก่อน probabilistic** | ผลลัพธ์ต้องซ้ำได้ (กฎ) ก่อนจะเติมความยืดหยุ่น (AI) |

---

## 2. ภาพรวม 3 ชั้น (Macro Architecture)

```
                        ┌───────────────────────────────┐
                        │        ผู้ใช้ (Users)          │
                        │   👩‍🏫 ครู      👨‍🎓 นักเรียน       │
                        └───────────────┬───────────────┘
                                        │
        ┌───────────────────────────────┼───────────────────────────────┐
        │                               │                               │
        ▼                               ▼                               ▼
┌────────────────┐            ┌────────────────┐            ┌────────────────┐
│ 1. KNOWLEDGE   │            │ 2. ASSESSMENT  │            │ 3. EXPRESS     │
│ (สมองที่สอง)    │            │ (วัดผล)         │            │ (ใช้จริง)       │
│                │            │                │            │                │
│ botany-xambrain│            │  ExamFlow      │            │ Food           │
│ vault          │  export    │  (เว็บ)         │            │ Recommender    │
│ Markdown+JSON  │ ─────────► │ test.xambrain  │            │ index.html     │
│                │  qbank     │ .com           │            │ recommender.js │
│ Teacher/Student│ ◄───────── │                │            │                │
│ App (.exe)     │  results   │ ตรวจ+ตัวชี้วัด  │            │ กฎ + menu-db   │
└───────┬────────┘            └────────────────┘            └───────┬────────┘
        │                                                           │
        │                  ┌────────────────────┐                  │
        └─────────────────►│ AI LAYER (optional) │◄─────────────────┘
                           │ ติวเตอร์ / ร่างข้อสอบ │
                           │ Gemini | Ollama     │
                           │ (provider-agnostic) │
                           └────────────────────┘
```

**สถานะ:** ชั้น 1 ✅ (repo นี้) · ชั้น 2 ✅ (ExamFlow ออนไลน์) · ชั้น 3 ✅ (prototype) · AI ✅ (RAG ติวเตอร์ในแอป)

---

## 3. ชั้นที่ 1 — Knowledge (สมองที่สอง)

### 3.1 หน้าที่
แหล่งความจริงเดียว (single source of truth) ของเนื้อหาทั้งหมด — ครูเป็นเจ้าของ

### 3.2 องค์ประกอบ

```
botany-xambrain/ (vault)
├── wiki/               concept nodes (Markdown + YAML frontmatter)
│   ├── glossary/       คลังคำศัพท์ 3 ภาษา
│   ├── questions/      qbank L1/L2/L3  ← (จะสร้าง)
│   ├── assessment/     master-assessment  ← (จะสร้าง)
│   └── menus/          วิเคราะห์เมนูเชิงลึก
├── data/               ฐานข้อมูลเครื่องอ่านได้
│   ├── menu-db.json         (schema 0.2)
│   ├── symptom-element.json
│   └── age-food.json
├── assets/             รูปภาพ
└── raw/                PDF ต้นฉบับ
```

### 3.3 รูปแบบข้อมูล (Data Contracts)

**Concept node** (Markdown):
```yaml
---
title: string
id: string          # เช่น S4, U3, T1, AX1
type: concept | source | overview | meta
layer: structure | utilization | thai-traditional
prerequisites: [id...]
source_type: certified | external
tags: [...]
---
```

**qbank item** (จะกำหนดใน `wiki/questions/`):
```yaml
question: string
level: L1 | L2 | L3           # ↔ Botany Literacy
concept: [[wikilink]]          # โยงกลับ concept node
indicator: string              # ตัวชี้วัด (ใช้ map กับ ExamFlow)
choices: [ก, ข, ค, ง]
answer: index
explain: string
```

### 3.4 เครื่องมือเข้าถึง vault
- **นักพัฒนา/ผู้เชี่ยวชาญ:** Obsidian (เปิด vault ตรงๆ)
- **ครูทั่วไป:** Teacher App (.exe) — ดูหัวข้อ 6
- **โปรแกรม:** อ่าน Markdown/JSON โดยตรง (parser)

---

## 4. ชั้นที่ 2 — Assessment (ExamFlow)

### 4.1 สถานะ
เว็บแอปแยกต่างหาก ออนไลน์แล้วที่ [test.xambrain.com](https://test.xambrain.com)

### 4.2 หน้าที่
- ครูสร้างข้อสอบ · อัปโหลดเฉลย (PDF/Excel)
- รับกระดาษคำตอบ → ตรวจอัตโนมัติ
- สรุปผลตาม **ตัวชี้วัด (indicator)**

### 4.3 จุดเชื่อมกับชั้นความรู้ (Integration Contract)

```
Vault qbank (Markdown)
      │  workflow "ผลิตข้อสอบ" (CLAUDE.md #5)
      ▼
export → .xlsx / .pdf   (รูปแบบที่ ExamFlow รับได้)
      │
      ▼
ExamFlow: import → สอบ → ตรวจ → รายงานตัวชี้วัด
      │
      ▼  (อนาคต) ส่งผลกลับ
Vault/Teacher App: แสดง "จุดอ่อนตาม Botany Literacy"
```

**สัญญาข้อมูลที่ต้อง fix:** mapping ระหว่าง `level (L1/L2/L3)` และ `indicator` ใน 2 ระบบต้องตรงกัน
→ ใช้ **Botany Literacy 5 ระดับ** เป็นภาษากลาง

---

## 5. ชั้นที่ 3 — Express (เครื่องมือใช้จริง)

### 5.1 Food Recommender
ตัวอย่างการนำความรู้ไปใช้จริง (Botany Literacy ระดับ Using/Connecting)

```
ฟอร์ม (index.html)
  น้ำหนัก/ส่วนสูง → BMI
  อายุ            → อายุสมุฏฐาน (age-food.json)
  อาการ           → ธาตุ/รส (symptom-element.json)
        │
        ▼
recommender.js  (กฎ + คะแนน — deterministic, ไม่มี AI)
        │
        ▼
จัดอันดับ menu-db.json (124 เมนู) → เมนูแนะนำ + เหตุผล + วิเคราะห์ Tier 2
```

**หลักสำคัญ:** LLM **ไม่เคย** เลือกเมนู — เลือกด้วยกฎเสมอ เพื่อให้ปลอดภัยและซ้ำได้

---

## 6. Teacher/Student App (.exe) — การนำส่ง (Delivery)

### 6.1 เทคโนโลยีที่เลือก: **Tauri 2**

| เกณฑ์ | Tauri 2 | Electron | เหตุผลที่เลือก Tauri |
|-------|---------|----------|----------------------|
| ขนาดไฟล์ | ~10–20 MB | ~120 MB+ | ครูดาวน์โหลด/ติดตั้งง่าย |
| ติดตั้ง | `.exe`/`.msi` เดียว | `.exe` ใหญ่ | ตรงเป้า "ไฟล์เดียว" |
| UI | ใช้ web (HTML/JS เดิม) | web เหมือนกัน | reuse `index.html` ได้ |
| ฝัง vault | ได้ (bundle resource) | ได้ | เนื้อหาไปพร้อมแอป |

### 6.2 โครงแอป

```
app/
├── src/                UI ภาษาไทย (reuse recommender + markdown viewer)
├── src-tauri/
│   ├── tauri.conf.json ตั้งชื่อแอป, icon, installer (NSIS/MSI)
│   └── src/            Rust: อ่านไฟล์ vault, เปิดลิงก์ ExamFlow
└── vault/              copy ของ wiki/ + data/ (bundle ตอน build)
```

### 6.3 สิ่งที่แอปทำ (MVP)
- แสดงรายการบทเรียน (ซ้าย) + อ่าน Markdown + รูป (กลาง)
- ปุ่ม「เปิด ExamFlow」→ เว็บ browser
- (เฟสถัดไป) แก้/เพิ่มบท · ร่าง qbank · export

### 6.4 การอัปเดตเนื้อหา (ครูไม่เก่งคอม)
```
วิธี 1: ติดตั้ง .exe เวอร์ชันใหม่ทับ         (ง่ายสุด, เริ่มด้วยวิธีนี้)
วิธี 2: ปุ่ม "อัปเดตบทเรียน" ในแอป           (ดาวน์โหลด vault bundle อัตโนมัติ — ภายหลัง)
```
**ห้าม** ให้ครูใช้ `git pull` — ไม่เหมาะกลุ่มเป้าหมาย

---

## 7. AI Layer — ติวเตอร์แชท (RAG ในแอป)

**สถานะ:** ✅ MVP พร้อมใช้งาน (branch `feature/ai-tutor-chat`)

### 7.1 สถาปัตยกรรม RAG (Retrieval-Augmented Generation)

```
scripts/build-embeddings.py
  wiki/*.md → chunk ตามหัวข้อ ##
       │  GEMINI_API_KEY
       ▼
  data/embeddings.json  [{id, file, title, heading, text, vector[768]}, ...]
       │ bundle ใน .exe (Tauri resources)
       ▼
  Rust (ai.rs)
    load_index() → AiState (Mutex<Vec<EmbeddingChunk>>)
       │
  [user question]
       │ gemini_embed()  → vec[768]
       ▼
  cosine similarity → top-5 chunks
       │ assemble prompt (system + context + question)
       ▼
  gemini_chat() → answer + citations[]
       │
  Frontend (chat.ts)
    แสดงคำตอบ + ชิปอ้างอิง
    คลิกชิป → เปิดบทเรียน (loadWikiFile)
```

### 7.2 ไฟล์ที่เกี่ยวข้อง

| ไฟล์ | หน้าที่ |
|------|---------|
| `scripts/build-embeddings.py` | สร้าง `data/embeddings.json` (รันก่อน build ครั้งแรก) |
| `data/embeddings.json` | ดัชนี vector (ไม่ commit — gitignore) |
| `app/src-tauri/src/ai.rs` | Rust: load index, set/get API key, ai_chat command |
| `app/src/chat.ts` | Frontend: พาเนลแชต, modal ตั้ง key, แสดงคำตอบ+citation |
| `app/src/main.ts` | mount chat panel, wire navigate |

### 7.3 บทบาทที่อนุญาต

| บทบาท | ผู้ใช้ | เงื่อนไข |
|-------|-------|----------|
| ร่างข้อสอบ L1 จาก concept | ครู | ครูต้องตรวจก่อนใช้ |
| อธิบายเฉลย / ชี้กลับ concept | นักเรียน | อ้างจาก vault เท่านั้น |
| ติวเตอร์ถาม-ตอบ | นักเรียน | ตอบเฉพาะในหลักสูตร ไม่ออกนอกขอบเขต |
| **ห้าม** | — | เลือกเมนูเอง · ตัดเกรด · วินิจฉัยโรค |

### 7.4 ความปลอดภัย API Key
- เก็บใน `app_config_dir()/config.json` (เครื่องผู้ใช้เท่านั้น)
- Fallback: env `GEMINI_API_KEY` (สำหรับ dev/CI)
- ไม่มีการส่ง key ผ่าน frontend → Rust เท่านั้นที่เรียก Gemini

### 7.5 LangGraph — เมื่อไหร่จึงจำเป็น
```
แชท 1 รอบ / if-else           → ไม่ต้องใช้ (ปัจจุบัน)
แชทหลายขั้น + หลาย tool + memory ยาว → ค่อยพิจารณา LangGraph
```

---

## 8. ความเป็นส่วนตัวและข้อมูล (Privacy / PDPA)

| ข้อมูล | เก็บที่ไหน (ค่าเริ่มต้น) | หมายเหตุ |
|--------|------------------------|----------|
| เนื้อหาบทเรียน | เครื่องครู (vault) | ไม่ใช่ข้อมูลส่วนบุคคล |
| คำตอบ/ประวัติเรียนของนักเรียน | เครื่องผู้ใช้ (localStorage/ไฟล์) | local-first ลดภาระ PDPA |
| ข้อมูลสอบ | ExamFlow (server) | ต้องมี consent + นโยบายความเป็นส่วนตัว |
| ข้อมูลสุขภาพ (recommender) | ไม่เก็บ server โดยค่าเริ่มต้น | อาการ/น้ำหนัก = ข้อมูลอ่อนไหว |

**ถ้าจะเก็บข้อมูลสุขภาพ/สอบบน server:** ต้องมีนโยบายความเป็นส่วนตัว, ความยินยอม, สิทธิลบข้อมูล, ระบุผู้ควบคุมข้อมูล ก่อนเปิดใช้จริง

---

## 9. สถาปัตยกรรมการนำส่ง (Deployment View)

```
┌──────────────────────────────────────────────────────────────┐
│ เครื่องผู้ใช้ (ครู/นักเรียน)                                     │
│  • Teacher/Student App (.exe)  ── อ่าน vault ในเครื่อง          │
│  • (ทางเลือก) Ollama            ── AI offline                   │
└───────────────┬──────────────────────────────────────────────┘
                │ HTTPS
                ▼
┌──────────────────────────────────────────────────────────────┐
│ Cloud                                                          │
│  • ExamFlow            test.xambrain.com   (สอบ/ตรวจ/รายงาน)   │
│  • Express (ทางเลือก)  food.xambrain.com   (แนะนำเมนูสาธารณะ)   │
│  • AI API (ทางเลือก)   Gemini Flash        (อธิบาย/ติว)        │
└──────────────────────────────────────────────────────────────┘
                ▲
                │ git local only (ผู้พัฒนา)
┌───────────────┴──────────────────────────────────────────────┐
│ เครื่องผู้พัฒนา (คุณ)                                          │
│  • botany-xambrain repo (แหล่งความจริง)                        │
│  • build .exe · export qbank · deploy เว็บ                     │
└──────────────────────────────────────────────────────────────┘
```

---

## 10. แผนพัฒนาตามสถาปัตยกรรม (Phasing)

| เฟส | ส่ง | ชั้นที่แตะ |
|-----|-----|-----------|
| **0 (ทำแล้ว)** | vault ความรู้ + ExamFlow + recommender prototype | 1, 2, 3 |
| **1** | qbank หัวข้อแรก + mapping ตัวชี้วัด ↔ Botany Literacy + export → ExamFlow | 1↔2 |
| **2** | Teacher App (.exe) MVP: อ่าน vault + ปุ่มไป ExamFlow | 1 (delivery) |
| **3** | ขยายเมนู Tier 2 + deploy Express เป็น URL สาธารณะ | 3 |
| **4** | Student App/เว็บ: เรียนเอง + ฝึกสอบ + เก็บผล local | 1↔2 |
| **5** | AI ติวเตอร์ (provider-agnostic) จาก vault ของครู | AI |

---

## 11. สรุปการตัดสินใจสำคัญ (Key Decisions)

| การตัดสินใจ | เลือก | เหตุผล |
|-------------|-------|--------|
| แหล่งความจริง | Markdown + JSON (plain text) | เปิดได้ทุกที่ · git · ไม่ผูก vendor |
| แอปครู | Tauri 2 → `.exe` ไฟล์เดียว | เล็ก · ติดตั้งง่าย · reuse web UI |
| Obsidian | เป็น **ทางเลือก** ผู้เชี่ยวชาญ ไม่ฝังในแอป | Obsidian ไม่ open source · ครูทั่วไปไม่จำเป็น |
| วัดผล | ExamFlow (เว็บ, มีแล้ว) | ไม่สร้างใหม่ · เชื่อมด้วย export |
| เลือกเมนู | กฎ (recommender.js) | ปลอดภัย · ซ้ำได้ · ไม่ hallucinate |
| AI | provider-agnostic, เพิ่มทีหลัง | หลีกเลี่ยง lock-in · ยังไม่จำเป็นตอนนี้ |
| LangGraph | ยังไม่ใช้ | flow ยังไม่ซับซ้อนพอ |
