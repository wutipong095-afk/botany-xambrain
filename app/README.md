# Xambrain Teacher App

แอปเดสก์ท็อป **การทำสมองที่สอง — ครู** (Tauri 2)

- อ่านบทเรียนจาก `../wiki/` (โหมด dev) หรือ `resources/vault/wiki/` (ตอน build)
- **Markdown renderer** — หัวข้อ ตาราง รายการ + **คลิก [[wikilink]]** ไปบทอื่นได้
- ปุ่มเปิด [ExamFlow](https://test.xambrain.com) สำหรับสร้าง/ตรวจข้อสอบ

## ความต้องการของระบบ (Windows)

1. [Node.js](https://nodejs.org/) 18+
2. [Rust](https://www.rust-lang.org/tools/install) (rustup)
3. [Prerequisites ของ Tauri](https://tauri.app/start/prerequisites/) — Visual Studio Build Tools + WebView2 (Windows 11 มี WebView2 แล้ว)

ติดตั้ง Rust (ครั้งเดียว):

```powershell
winget install Rustlang.Rustup
# หรือดาวน์โหลดจาก https://rustup.rs/
rustup default stable
```

## คำสั่ง

```powershell
cd app
npm install
npm run tauri dev      # รันแอป (dev)
npm run tauri build    # สร้าง .exe installer
```

ไฟล์ติดตั้งจะอยู่ที่ `app/src-tauri/target/release/bundle/`

## ติวเตอร์ AI (RAG)

พาเนลแชตทางขวาของแอปให้ถามคำถามด้านพฤกษศาสตร์ — คำตอบอ้างอิงเฉพาะ vault

### ขั้นตอน (ทำครั้งแรกและทุกครั้งที่เนื้อหาเปลี่ยน)

**1. สร้างดัชนี embeddings**

```powershell
# จาก root ของ repo (ไม่ใช่ app/)
$env:GEMINI_API_KEY = "AIza..."
python scripts/build-embeddings.py
# จะได้ data/embeddings.json (~หลาย MB)
```

**2. ตั้งค่า API key ในแอป**

เปิดแอป → คลิกปุ่ม 🔑 ที่มุมพาเนลแชต → ใส่ Gemini API key → บันทึก

Key เก็บในเครื่องผู้ใช้ที่ `%APPDATA%\com.xambrain.teacher\config.json` — ไม่ส่งออกนอกจาก Gemini API

### รับ API key ฟรี

1. ไปที่ [aistudio.google.com](https://aistudio.google.com/)
2. คลิก "Get API key" → สร้าง key ใหม่
3. Copy key ที่ขึ้นต้นด้วย `AIza...`

### การทำงาน (RAG flow)

```
คำถาม → embed (Gemini) → cosine top-5 chunks → prompt → Gemini Flash → คำตอบ + ชิปอ้างอิง
```

คลิกชิปอ้างอิง → เปิดบทเรียนนั้นในแอปทันที

## โครงสร้าง

```
app/
├── src/
│   ├── main.ts       ควบคุม UI หลัก + mount chat
│   ├── chat.ts       พาเนลแชต AI (RAG frontend)
│   └── markdown.ts   render Markdown + wikilinks
├── src-tauri/
│   └── src/
│       ├── lib.rs    คำสั่ง: wiki, examflow, re-export AI commands
│       └── ai.rs     API key storage, embed, cosine, Gemini chat
└── resources/vault/  (อนาคต) copy wiki+data ตอน build สำหรับครูที่ไม่มี repo
```

## หมายเหตุ

- โหมด dev อ่าน vault + embeddings จาก root ของ repo โดยอัตโนมัติ
- **Release (.exe)** ฝัง `wiki/` + `assets/` + `data/` ใน installer — ครูไม่ต้องมี repo
- มุมล่าง sidebar แสดง `[bundled]` หรือ `[dev]` ตามแหล่งข้อมูล
- รองรับ `![[โฟลเดอร์/รูป.png|500]]` — โหลดจาก `assets/` ใน vault (ตัดต่อจาก PDF)
- ดูสถาปัตยกรรมรวมที่ [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
