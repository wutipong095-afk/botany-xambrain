use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;

// ---- types ----------------------------------------------------------------

#[derive(Serialize, Deserialize, Clone)]
pub struct EmbeddingChunk {
    pub id: String,
    pub file: String,
    pub title: String,
    pub heading: String,
    pub text: String,
    pub vector: Vec<f32>,
}

#[derive(Serialize)]
pub struct ChatResponse {
    pub answer: String,
    pub citations: Vec<Citation>,
}

#[derive(Serialize, Clone)]
pub struct Citation {
    pub title: String,
    pub file: String,
    pub heading: String,
}

#[derive(Deserialize)]
struct GeminiEmbedResponse {
    embedding: GeminiEmbedValues,
}

#[derive(Deserialize)]
struct GeminiEmbedValues {
    values: Vec<f32>,
}

#[derive(Deserialize)]
struct GeminiGenerateResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    content: GeminiContent,
}

#[derive(Deserialize)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Deserialize)]
struct GeminiPart {
    text: String,
}

// ---- in-memory index + shared HTTP client ----------------------------------

pub struct AiState {
    pub index: Mutex<Vec<EmbeddingChunk>>,
    /// Client ใช้ร่วมกัน (connection pool) — Clone ได้ ราคาถูก
    client: reqwest::Client,
}

impl AiState {
    pub fn new() -> Self {
        AiState {
            index: Mutex::new(Vec::new()),
            client: reqwest::Client::new(),
        }
    }
}

// ---- key storage -----------------------------------------------------------

fn config_path(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_config_dir()
        .map(|p| p.join("config.json"))
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_api_key(app: AppHandle, key: String) -> Result<(), String> {
    let path = config_path(&app)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let obj = serde_json::json!({ "gemini_api_key": key });
    fs::write(&path, obj.to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn get_api_key_status(app: AppHandle) -> bool {
    read_api_key(&app).map(|k| !k.is_empty()).unwrap_or(false)
}

pub fn read_api_key(app: &AppHandle) -> Result<String, String> {
    if let Ok(path) = config_path(app) {
        if path.exists() {
            if let Ok(raw) = fs::read_to_string(&path) {
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(&raw) {
                    if let Some(k) = v.get("gemini_api_key").and_then(|v| v.as_str()) {
                        if !k.is_empty() {
                            return Ok(k.to_string());
                        }
                    }
                }
            }
        }
    }
    std::env::var("GEMINI_API_KEY").map_err(|_| "API key ยังไม่ได้ตั้งค่า".into())
}

// ---- load index ------------------------------------------------------------

fn vault_data_dir(app: &AppHandle) -> Option<PathBuf> {
    #[cfg(debug_assertions)]
    {
        let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let dev_data = manifest.join("..").join("..").join("data");
        if dev_data.join("embeddings.json").exists() {
            return Some(dev_data.canonicalize().unwrap_or(dev_data));
        }
    }
    app.path()
        .resolve("vault/data", BaseDirectory::Resource)
        .ok()
        .filter(|p| p.join("embeddings.json").exists())
}

pub fn load_index(app: &AppHandle, state: &AiState) {
    let Some(data_dir) = vault_data_dir(app) else {
        return;
    };
    let path = data_dir.join("embeddings.json");
    let Ok(raw) = fs::read_to_string(&path) else {
        return;
    };
    let Ok(chunks) = serde_json::from_str::<Vec<EmbeddingChunk>>(&raw) else {
        return;
    };
    if let Ok(mut idx) = state.index.lock() {
        *idx = chunks;
    }
}

// ---- cosine similarity -----------------------------------------------------

fn cosine(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let nb: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if na == 0.0 || nb == 0.0 { 0.0 } else { dot / (na * nb) }
}

// ---- UTF-8 safe truncation -------------------------------------------------

fn truncate_chars(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

// ---- Gemini helpers (async) -------------------------------------------------

const EMBED_MODEL: &str = "models/gemini-embedding-001";
const CHAT_MODEL: &str = "models/gemini-2.5-flash";
const GEMINI_V1: &str = "https://generativelanguage.googleapis.com/v1";
const GEMINI_V1BETA: &str = "https://generativelanguage.googleapis.com/v1beta";

async fn gemini_embed(client: &reqwest::Client, text: &str, api_key: &str) -> Result<Vec<f32>, String> {
    let url = format!("{GEMINI_V1}/{EMBED_MODEL}:embedContent?key={api_key}");
    let body = serde_json::json!({
        "model": EMBED_MODEL,
        "content": { "parts": [{ "text": text }] }
    });
    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("embed request: {e}"))?;
    if !resp.status().is_success() {
        let code = resp.status().as_u16();
        let txt = resp.text().await.unwrap_or_default();
        return Err(format!("Gemini embed {code}: {}", truncate_chars(&txt, 300)));
    }
    let parsed: GeminiEmbedResponse = resp.json().await.map_err(|e| format!("embed parse: {e}"))?;
    Ok(parsed.embedding.values)
}

async fn gemini_chat(client: &reqwest::Client, prompt: &str, api_key: &str) -> Result<String, String> {
    let url = format!("{GEMINI_V1BETA}/{CHAT_MODEL}:generateContent?key={api_key}");
    let body = serde_json::json!({
        "contents": [{ "parts": [{ "text": prompt }] }],
        "generationConfig": {
            "temperature": 0.3,
            "maxOutputTokens": 8192
        }
    });
    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("chat request: {e}"))?;
    if !resp.status().is_success() {
        let code = resp.status().as_u16();
        let txt = resp.text().await.unwrap_or_default();
        return Err(format!("Gemini chat {code}: {}", truncate_chars(&txt, 300)));
    }
    let parsed: GeminiGenerateResponse = resp.json().await.map_err(|e| format!("chat parse: {e}"))?;
    parsed
        .candidates
        .into_iter()
        .next()
        .and_then(|c| c.content.parts.into_iter().next())
        .map(|p| p.text)
        .ok_or_else(|| "ไม่มีคำตอบจาก Gemini".into())
}

// ---- main command (async) --------------------------------------------------

const SYSTEM_PROMPT: &str = r"คุณคือติวเตอร์ด้านพฤกษศาสตร์พื้นบ้านและสุขภาพไทย ตอบเป็นภาษาไทย ฉลาด รอบรู้ และช่วยผู้เรียนอย่างเต็มที่

หลักการตอบ:
- ใช้ 'เนื้อหาอ้างอิง' ด้านล่างเป็นฐานหลักก่อนเสมอ และอ้างชื่อหัวข้อที่ใช้
- ถ้าเนื้อหาอ้างอิงไม่พอ หรือไม่เกี่ยวข้อง ให้ใช้ความรู้ทั่วไปด้านพฤกษศาสตร์/สมุนไพรไทยของคุณตอบต่อได้เลย และบอกสั้นๆ ว่าส่วนนี้เป็นความรู้ทั่วไป (นอกหลักสูตร)
- ถ้าเป็นคำสั่งให้สร้างงาน (ออกข้อสอบ สรุป ทำ flashcard เปรียบเทียบ ฯลฯ) ให้ทำตามคำสั่งนั้นทันที ห้ามปฏิเสธว่า 'ไม่มีข้อมูล' — ถ้าคำสั่งกว้างไป (เช่นไม่ได้ระบุหัวข้อ) ให้เลือกหัวข้อที่เหมาะสมจากเนื้อหาอ้างอิงมาทำ หรือถามกลับสั้นๆ ว่าต้องการหัวข้อใด
- ตอบให้กระชับ ชัดเจน เป็นระบบ

ข้อควรระวัง: ห้ามวินิจฉัยหรือสั่งการรักษาโรค — ให้เป็นข้อมูลส่งเสริมสุขภาพเชิงป้องกันเท่านั้น";

#[tauri::command]
pub async fn ai_chat(
    app: AppHandle,
    state: tauri::State<'_, AiState>,
    question: String,
) -> Result<ChatResponse, String> {
    let api_key = read_api_key(&app)?;
    let client = state.client.clone();

    // embed คำถาม (async — ไม่บล็อก UI thread)
    let q_vec = gemini_embed(&client, &question, &api_key).await?;

    // คำนวณ top-5 + clone ข้อมูลที่ต้องใช้ ก่อน drop Mutex lock
    const TOP_N: usize = 5;
    let (context, citations) = {
        let index = state.index.lock().map_err(|e| e.to_string())?;
        if index.is_empty() {
            return Err("ยังไม่มีดัชนี embeddings — กรุณารัน scripts/build-embeddings.py ก่อน".into());
        }

        let mut scored: Vec<(f32, usize)> = index
            .iter()
            .enumerate()
            .map(|(i, c)| (cosine(&q_vec, &c.vector), i))
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let top: Vec<&EmbeddingChunk> = scored.iter().take(TOP_N).map(|(_, i)| &index[*i]).collect();

        let ctx = top
            .iter()
            .enumerate()
            .map(|(n, c)| {
                let hdr = if c.heading.is_empty() {
                    format!("[{}] {}", n + 1, c.title)
                } else {
                    format!("[{}] {} — {}", n + 1, c.title, c.heading)
                };
                format!("{hdr}\n{}\n", c.text)
            })
            .collect::<Vec<_>>()
            .join("\n---\n");

        let cits: Vec<Citation> = top
            .iter()
            .map(|c| Citation {
                title: c.title.clone(),
                file: c.file.clone(),
                heading: c.heading.clone(),
            })
            .collect();

        (ctx, cits)
        // Mutex lock ถูก drop ที่นี่ ก่อนเรียก network
    };

    let prompt = format!(
        "{SYSTEM_PROMPT}\n\n=== เนื้อหาอ้างอิง ===\n{context}\n=== คำถาม ===\n{question}"
    );

    // เรียก Gemini หลัง drop lock แล้ว — ไม่บล็อกผู้เรียกอื่น
    let answer = gemini_chat(&client, &prompt, &api_key).await?;

    Ok(ChatResponse { answer, citations })
}
