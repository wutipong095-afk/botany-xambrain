use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Manager};

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

/// ข้อความหนึ่งบรรทัดในประวัติแชต (ส่งมาจาก frontend)
#[derive(Deserialize)]
pub struct ChatMessage {
    /// "user" หรือ "assistant"/"model"
    pub role: String,
    pub content: String,
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
    #[serde(default)]
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize)]
struct GeminiCandidate {
    /// อาจหายเมื่อ thinking กินโควตาจนไม่เหลือ text (finishReason = MAX_TOKENS)
    content: Option<GeminiContent>,
    #[serde(rename = "finishReason")]
    finish_reason: Option<String>,
}

#[derive(Deserialize)]
struct GeminiContent {
    /// บาง response มี content แต่ไม่มี parts
    #[serde(default)]
    parts: Option<Vec<GeminiPart>>,
}

#[derive(Deserialize)]
struct GeminiPart {
    /// thinking part อาจไม่มี text ธรรมดา
    text: Option<String>,
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

/// data/ ที่มี embeddings.json — dev ใช้ repo root, release ใช้ bundled resource
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

/// โหลดดัชนี embeddings เข้าหน่วยความจำ
/// (ถ้าไม่มี/อ่านไม่ได้ → ล้างดัชนีเป็นว่าง)
pub fn load_index(app: &AppHandle, state: &AiState) {
    let chunks = vault_data_dir(app)
        .and_then(|d| fs::read_to_string(d.join("embeddings.json")).ok())
        .and_then(|raw| serde_json::from_str::<Vec<EmbeddingChunk>>(&raw).ok())
        .unwrap_or_default();
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

// ---- keyword scoring (hybrid search) ---------------------------------------
// เสริม cosine ด้วยการ match คำตรงตัว — สำคัญกับชื่อพืช/ชื่อวิทยาศาสตร์ที่ต้องเป๊ะ
// เพราะ embedding เพียงอย่างเดียวบางทีพลาดคำเฉพาะ (เช่น "Curcuma longa", "ชบา")

const U0E00: char = '\u{0E00}';
const U0E7F: char = '\u{0E7F}';

fn is_thai(c: char) -> bool {
    (U0E00..=U0E7F).contains(&c)
}

/// token ภาษาอังกฤษ/ละติน (ยาว >= 3) — ใช้กับชื่อวิทยาศาสตร์/ศัพท์เทคนิค
fn ascii_terms(s: &str) -> Vec<String> {
    s.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| t.len() >= 3)
        .map(|t| t.to_ascii_lowercase())
        .collect()
}

/// character trigram ของเฉพาะอักษรไทย — ใช้วัดการซ้อนทับของคำไทย
/// (ภาษาไทยไม่มีช่องว่างระหว่างคำ จึงใช้ n-gram แทน tokenizer)
fn thai_trigrams(s: &str) -> HashSet<String> {
    let thai: Vec<char> = s.chars().filter(|c| is_thai(*c)).collect();
    let mut set = HashSet::new();
    if thai.len() < 3 {
        if !thai.is_empty() {
            set.insert(thai.iter().collect::<String>());
        }
        return set;
    }
    for w in thai.windows(3) {
        set.insert(w.iter().collect::<String>());
    }
    set
}

/// คะแนน keyword 0..1 = สัดส่วนคำ/ไตรแกรมของคำถามที่พบในเนื้อหา chunk
fn keyword_score(query: &str, text: &str) -> f32 {
    let text_l = text.to_lowercase();

    // ASCII terms (ชื่อวิทยาศาสตร์ ฯลฯ)
    let terms = ascii_terms(query);
    let ascii_score = if terms.is_empty() {
        0.0
    } else {
        let hit = terms.iter().filter(|t| text_l.contains(*t)).count();
        hit as f32 / terms.len() as f32
    };

    // Thai trigram overlap
    let q_tri = thai_trigrams(query);
    let thai_score = if q_tri.is_empty() {
        0.0
    } else {
        let t_tri = thai_trigrams(&text_l);
        let inter = q_tri.iter().filter(|g| t_tri.contains(*g)).count();
        inter as f32 / q_tri.len() as f32
    };

    ascii_score.max(thai_score)
}

// ---- UTF-8 safe truncation -------------------------------------------------

fn truncate_chars(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

// ---- Gemini helpers (async) -------------------------------------------------

const EMBED_MODEL: &str = "models/gemini-embedding-001";
/// โมเดลเริ่มต้น — เร็ว ประหยัด เหมาะกับคำถามทั่วไป
const CHAT_MODEL: &str = "models/gemini-2.5-flash";
/// โมเดล "คิดลึก" — ช้ากว่าแต่เก่งเหตุผลซับซ้อน (ใช้เมื่อผู้ใช้เปิดโหมด Pro)
const CHAT_MODEL_PRO: &str = "models/gemini-2.5-pro";
const GEMINI_V1: &str = "https://generativelanguage.googleapis.com/v1";
const GEMINI_V1BETA: &str = "https://generativelanguage.googleapis.com/v1beta";

/// งบ token สำหรับ "การคิด" ของ Gemini 2.5
/// จำกัดไว้ 4096 (ไม่ใช้ -1 = dynamic) เพื่อกัน thinking กินโควตาจนไม่เหลือ text
const THINKING_BUDGET: i32 = 4096;
/// เพดาน token รวม (thinking + คำตอบ) — เผื่อ thinking 4096 แล้วยังเหลือพื้นที่ตอบ
const MAX_OUTPUT_TOKENS: i32 = 16384;

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

async fn gemini_chat(
    client: &reqwest::Client,
    model: &str,
    system: &str,
    contents: serde_json::Value,
    api_key: &str,
) -> Result<String, String> {
    let url = format!("{GEMINI_V1BETA}/{model}:generateContent?key={api_key}");
    let body = serde_json::json!({
        "systemInstruction": { "parts": [{ "text": system }] },
        "contents": contents,
        "generationConfig": {
            "temperature": 0.3,
            "maxOutputTokens": MAX_OUTPUT_TOKENS,
            "thinkingConfig": { "thinkingBudget": THINKING_BUDGET }
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
    let candidate = parsed
        .candidates
        .into_iter()
        .next()
        .ok_or_else(|| "ไม่มีคำตอบจาก Gemini (ไม่มี candidates)".to_string())?;

    let finish = candidate.finish_reason.as_deref().unwrap_or("");
    let text = candidate
        .content
        .as_ref()
        .and_then(|c| c.parts.as_ref())
        .map(|parts| {
            parts
                .iter()
                .filter_map(|p| p.text.as_deref())
                .filter(|t| !t.trim().is_empty())
                .collect::<Vec<_>>()
                .join("\n")
        })
        .unwrap_or_default();

    if !text.trim().is_empty() {
        // มีข้อความแล้ว — ถ้าโดนตัดกลางคัน บอกผู้ใช้ต่อท้าย
        if finish == "MAX_TOKENS" {
            return Ok(format!(
                "{text}\n\n_(คำตอบถูกตัดเพราะยาวเกินโควตา — ลองถามให้แคบลงหรือปิดโหมดคิดลึก)_"
            ));
        }
        return Ok(text);
    }

    // ไม่มี text เลย — แยกสาเหตุให้ผู้ใช้เข้าใจ
    Err(match finish {
        "MAX_TOKENS" => {
            "คำตอบยาวเกินโควตา (thinking กิน token จนไม่เหลือข้อความ) — ลองถามให้แคบลง หรือปิดโหมดคิดลึก".into()
        }
        "SAFETY" => "Gemini ปฏิเสธคำตอบด้วยเหตุผลความปลอดภัย — ลองปรับคำถาม".into(),
        "RECITATION" => "Gemini ปฏิเสธคำตอบเพราะอาจซ้ำเนื้อหาที่มีลิขสิทธิ์ — ลองถามใหม่".into(),
        other if !other.is_empty() => {
            format!("ไม่มีคำตอบจาก Gemini (finishReason: {other})")
        }
        _ => "ไม่มีคำตอบจาก Gemini".into(),
    })
}

// ---- main command (async) --------------------------------------------------

const SYSTEM_PROMPT: &str = r"คุณคือติวเตอร์ด้านพฤกษศาสตร์พื้นบ้านและสุขภาพไทย ตอบเป็นภาษาไทย ฉลาด รอบรู้ และช่วยผู้เรียนอย่างเต็มที่

หลักการตอบ:
- ใช้ 'เนื้อหาอ้างอิง' ด้านล่างเป็นฐานหลักก่อนเสมอ และอ้างชื่อหัวข้อที่ใช้
- ถ้าเนื้อหาอ้างอิงไม่พอ หรือไม่เกี่ยวข้อง ให้ใช้ความรู้ทั่วไปด้านพฤกษศาสตร์/สมุนไพรไทยของคุณตอบต่อได้เลย และบอกสั้นๆ ว่าส่วนนี้เป็นความรู้ทั่วไป (นอกหลักสูตร)
- ถ้าเป็นคำสั่งให้สร้างงาน (ออกข้อสอบ สรุป ทำ flashcard เปรียบเทียบ ฯลฯ) ให้ทำตามคำสั่งนั้นทันที ห้ามปฏิเสธว่า 'ไม่มีข้อมูล' — ถ้าคำสั่งกว้างไป (เช่นไม่ได้ระบุหัวข้อ) ให้เลือกหัวข้อที่เหมาะสมจากเนื้อหาอ้างอิงมาทำ หรือถามกลับสั้นๆ ว่าต้องการหัวข้อใด
- ตอบให้กระชับ ชัดเจน เป็นระบบ

ข้อควรระวัง: ห้ามวินิจฉัยหรือสั่งการรักษาโรค — ให้เป็นข้อมูลส่งเสริมสุขภาพเชิงป้องกันเท่านั้น";

/// จำนวนข้อความในประวัติที่ส่งกลับเข้าโมเดล (คู่ user/assistant ~ 3 รอบ)
const MAX_HISTORY: usize = 6;

#[tauri::command]
pub async fn ai_chat(
    app: AppHandle,
    state: tauri::State<'_, AiState>,
    question: String,
    history: Vec<ChatMessage>,
    deep: bool,
) -> Result<ChatResponse, String> {
    let api_key = read_api_key(&app)?;
    let client = state.client.clone();
    let model = if deep { CHAT_MODEL_PRO } else { CHAT_MODEL };

    // เก็บเฉพาะประวัติล่าสุด และตัดหัวให้เริ่มด้วย user เสมอ
    // (Gemini ต้องการ contents เริ่มด้วย role "user")
    let mut recent: Vec<&ChatMessage> = {
        let start = history.len().saturating_sub(MAX_HISTORY);
        history[start..].iter().collect()
    };
    while recent.first().map(|m| m.role != "user").unwrap_or(false) {
        recent.remove(0);
    }

    // สร้าง query สำหรับค้นหา: รวมคำถามก่อนหน้าเข้าไปด้วย เพื่อให้คำถามต่อเนื่อง
    // (เช่น "แล้วสรรพคุณมันล่ะ?") ค้นเจอเนื้อหาที่เกี่ยวข้อง
    let retrieval_query = match recent.iter().rev().find(|m| m.role == "user") {
        Some(prev) if !prev.content.trim().is_empty() => {
            format!("{}\n{question}", prev.content)
        }
        _ => question.clone(),
    };

    // embed query (async — ไม่บล็อก UI thread)
    let q_vec = gemini_embed(&client, &retrieval_query, &api_key).await?;

    // เลือก chunk ที่เกี่ยวข้อง: เอา top-N มาก่อน แล้วกรองด้วย relative threshold
    // (ตัดตัวที่คะแนนห่างจากตัวดีสุดมาก แต่คงตัวบนสุดไว้อย่างน้อย MIN_KEEP
    //  เผื่อคำถามแคบที่ match เนื้อหาน้อย จะได้ไม่เหลือ context ว่าง)
    const TOP_N: usize = 8;
    const MIN_KEEP: usize = 3;
    const REL_THRESHOLD: f32 = 0.80;
    let (context, citations) = {
        let index = state.index.lock().map_err(|e| e.to_string())?;
        if index.is_empty() {
            return Err("ยังไม่มีดัชนี embeddings — กรุณารัน scripts/build-embeddings.py ก่อน".into());
        }

        // hybrid: ผสมคะแนน cosine (ความหมาย) กับ keyword (คำตรงตัว)
        // ใช้ question ล่าสุดในการ match คำ (ไม่รวมคำถามก่อนหน้า กันคำแทนเป็น noise)
        const W_COS: f32 = 0.75;
        const W_KW: f32 = 0.25;
        let mut scored: Vec<(f32, usize)> = index
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let cos = cosine(&q_vec, &c.vector);
                let kw = keyword_score(&question, &c.text);
                (W_COS * cos + W_KW * kw, i)
            })
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let best = scored.first().map(|(s, _)| *s).unwrap_or(0.0);
        let cutoff = best * REL_THRESHOLD;
        let top: Vec<&EmbeddingChunk> = scored
            .iter()
            .take(TOP_N)
            .enumerate()
            .filter(|(rank, (score, _))| *rank < MIN_KEEP || *score >= cutoff)
            .map(|(_, (_, i))| &index[*i])
            .collect();

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

    // เทิร์นปัจจุบัน: แนบเนื้อหาอ้างอิง (ค้นใหม่ทุกครั้ง) เข้ากับคำถามล่าสุด
    let user_turn =
        format!("=== เนื้อหาอ้างอิง ===\n{context}\n=== คำถาม ===\n{question}");

    // สร้าง contents หลายเทิร์น: ประวัติเดิม + เทิร์นปัจจุบัน
    let mut contents: Vec<serde_json::Value> = recent
        .iter()
        .map(|m| {
            let role = if m.role == "assistant" || m.role == "model" {
                "model"
            } else {
                "user"
            };
            serde_json::json!({ "role": role, "parts": [{ "text": m.content }] })
        })
        .collect();
    contents.push(serde_json::json!({
        "role": "user",
        "parts": [{ "text": user_turn }]
    }));

    // เรียก Gemini หลัง drop lock แล้ว — ไม่บล็อกผู้เรียกอื่น
    let answer = gemini_chat(
        &client,
        model,
        SYSTEM_PROMPT,
        serde_json::Value::Array(contents),
        &api_key,
    )
    .await?;

    Ok(ChatResponse { answer, citations })
}
