# Xambrain Teacher App

แอปเดสก์ท็อป **การทำสมองที่สอง — ครู** (Tauri 2)

- อ่านบทเรียนจาก `../wiki/` (โหมด dev) หรือ `resources/vault/wiki/` (ตอน build)
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

## โครงสร้าง

```
app/
├── src/              UI (TypeScript + Vite)
├── src-tauri/        Rust backend + Tauri config
│   └── src/lib.rs    คำสั่ง: list_wiki_entries, read_wiki_file, examflow_url
└── resources/vault/  (อนาคต) copy wiki+data ตอน build สำหรับครูที่ไม่มี repo
```

## หมายเหตุ

- โหมด dev อ่าน vault จาก `botany-xambrain/wiki/` โดยอัตโนมัติ
- ยังไม่มี markdown renderer เต็มรูป — แสดง plain text (ตัด YAML frontmatter)
- ดูสถาปัตยกรรมรวมที่ [`../ARCHITECTURE.md`](../ARCHITECTURE.md)
