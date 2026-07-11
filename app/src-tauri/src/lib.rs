mod ai;
pub use ai::{set_api_key, get_api_key_status, ai_chat};

use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use tauri::path::BaseDirectory;

const EXAMFLOW_URL: &str = "https://test.xambrain.com";

#[derive(Serialize)]
struct WikiEntry {
    path: String,
    title: String,
}

// ---- content packs (multi-subject) ----------------------------------------
// วิชา built-in (ฝังมากับแอป) + วิชาที่ผู้ใช้โหลดเพิ่มลง {app_local_data}/packs/{id}/
// แต่ละ pack root มี wiki/ · assets/ · data/ อยู่ข้างใน

const BUILTIN_PACK_ID: &str = "botany";
const BUILTIN_PACK_NAME: &str = "พฤกษศาสตร์พื้นบ้าน";

/// path ของโฟลเดอร์เก็บแพ็ก — {app_local_data}/packs (ไม่แตะดิสก์)
fn packs_base_dir(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(app
        .path()
        .app_local_data_dir()
        .map_err(|e| e.to_string())?
        .join("packs"))
}

/// เหมือน packs_base_dir แต่สร้างโฟลเดอร์ให้ด้วย (ใช้เฉพาะตอนจะเขียน)
fn ensure_packs_base_dir(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = packs_base_dir(app)?;
    fs::create_dir_all(&dir).map_err(|e| format!("create packs dir: {e}"))?;
    Ok(dir)
}

fn active_pack_file(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(packs_base_dir(app)?.join("active.txt"))
}

/// id ของวิชาที่กำลังใช้งาน (default = built-in botany)
fn active_pack_id(app: &AppHandle) -> String {
    active_pack_file(app)
        .ok()
        .and_then(|p| fs::read_to_string(p).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| BUILTIN_PACK_ID.to_string())
}

fn validate_pack_id(id: &str) -> Result<(), String> {
    if id.is_empty()
        || id.contains("..")
        || id.contains('/')
        || id.contains('\\')
        || id.contains(':')
    {
        return Err("invalid pack id".into());
    }
    Ok(())
}

fn write_active_pack(app: &AppHandle, id: &str) -> Result<(), String> {
    validate_pack_id(id)?;
    ensure_packs_base_dir(app)?;
    let file = active_pack_file(app)?;
    fs::write(&file, id.as_bytes()).map_err(|e| format!("write active pack: {e}"))
}

/// root ของแพ็กที่โหลดมา (ต้องมี wiki/index.md) — None ถ้ายังไม่ได้ติดตั้ง
/// built-in botany ไม่ resolve จาก packs/ (กัน shadow กับโฟลเดอร์ packs/botany/)
fn installed_pack_root(app: &AppHandle, id: &str) -> Option<PathBuf> {
    validate_pack_id(id).ok()?;
    if id == BUILTIN_PACK_ID {
        return None;
    }
    let base = packs_base_dir(app).ok()?;
    let root = base.join(id);
    if !root.join("wiki").join("index.md").exists() {
        return None;
    }
    let base_canon = base.canonicalize().unwrap_or(base);
    let root_canon = root.canonicalize().unwrap_or_else(|_| root.clone());
    if !root_canon.starts_with(&base_canon) {
        return None;
    }
    Some(root_canon)
}

/// root ของวิชา built-in (botany) — dev repo หรือ bundled vault/
fn builtin_pack_root(app: &AppHandle) -> Option<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dev_root = manifest.join("..").join("..");

    // dev: repo root (../../)
    #[cfg(debug_assertions)]
    if dev_root.join("wiki").join("index.md").exists() {
        return Some(dev_root.canonicalize().unwrap_or(dev_root));
    }

    // release: bundled resource vault/
    if let Ok(p) = app.path().resolve("vault", BaseDirectory::Resource) {
        if p.join("wiki").join("index.md").exists() {
            return Some(p);
        }
    }

    // release-from-source fallback
    if dev_root.join("wiki").join("index.md").exists() {
        return Some(dev_root.canonicalize().unwrap_or(dev_root));
    }

    // local resources
    let local = manifest.join("resources").join("vault");
    if local.join("wiki").join("index.md").exists() {
        return Some(local.canonicalize().unwrap_or(local));
    }

    None
}

/// resolve root ของวิชาตาม id — built-in botany ใช้ bundled/dev เท่านั้น
fn resolve_pack_root(app: &AppHandle, id: &str) -> Result<PathBuf, String> {
    validate_pack_id(id)?;
    if id == BUILTIN_PACK_ID {
        return builtin_pack_root(app)
            .ok_or_else(|| format!("ไม่พบวิชา built-in: {BUILTIN_PACK_ID}"));
    }
    installed_pack_root(app, id).ok_or_else(|| format!("ไม่พบวิชา: {id}"))
}

/// cache ของวิชาที่ active — (id, root ที่ resolve แล้ว)
/// กันการอ่าน active.txt + canonicalize ซ้ำทุกครั้งที่โหลดรูป/บทเรียน
#[derive(Default)]
struct PackState {
    active: std::sync::Mutex<Option<(String, PathBuf)>>,
}

/// resolve วิชาที่ active จากดิสก์ — sync active.txt ถ้า id เก่า/ไม่ถูกต้อง
/// คืน (effective_id, root)
fn compute_pack_root(app: &AppHandle) -> Result<(String, PathBuf), String> {
    let id = active_pack_id(app);

    if validate_pack_id(&id).is_err() {
        if builtin_pack_root(app).is_some() {
            write_active_pack(app, BUILTIN_PACK_ID)?;
            let root = resolve_pack_root(app, BUILTIN_PACK_ID)?;
            return Ok((BUILTIN_PACK_ID.to_string(), root));
        }
        return Err("active pack id ไม่ถูกต้อง".into());
    }

    match resolve_pack_root(app, &id) {
        Ok(p) => Ok((id, p)),
        Err(e) if id != BUILTIN_PACK_ID => {
            // แพ็กใน active.txt ถูกลบ → reset เป็น built-in แทน fallback เงียบ ๆ
            if let Some(p) = builtin_pack_root(app) {
                write_active_pack(app, BUILTIN_PACK_ID)?;
                Ok((BUILTIN_PACK_ID.to_string(), p))
            } else {
                Err(format!("{e} (ไม่มี built-in vault สำรอง)"))
            }
        }
        Err(e) => Err(e),
    }
}

fn set_cached_pack(app: &AppHandle, id: String, root: PathBuf) {
    if let Ok(mut g) = app.state::<PackState>().active.lock() {
        *g = Some((id, root));
    }
}

/// root ของวิชาที่ active (อ่านจาก cache; cold ครั้งแรก compute จากดิสก์)
pub(crate) fn pack_root(app: &AppHandle) -> Result<PathBuf, String> {
    if let Ok(g) = app.state::<PackState>().active.lock() {
        if let Some((_, root)) = g.as_ref() {
            return Ok(root.clone());
        }
    }
    let (id, root) = compute_pack_root(app)?;
    set_cached_pack(app, id, root.clone());
    Ok(root)
}

fn vault_wiki_dir(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(pack_root(app)?.join("wiki"))
}

fn vault_root_dir(app: &AppHandle) -> Result<PathBuf, String> {
    pack_root(app)
}

fn vault_assets_dir(app: &AppHandle) -> Result<PathBuf, String> {
    Ok(vault_root_dir(app)?.join("assets"))
}

fn validate_relative_path(relative_path: &str) -> Result<(), String> {
    if relative_path.contains("..") || relative_path.contains(':') {
        return Err("invalid path".into());
    }
    Ok(())
}

fn resolve_asset_path(app: &AppHandle, relative_path: &str) -> Result<PathBuf, String> {
    validate_relative_path(relative_path)?;
    let assets = vault_assets_dir(app)?;
    let trimmed = relative_path.trim().trim_start_matches('/').replace('\\', "/");

    let candidates = [
        assets.join(&trimmed),
        assets.join(trimmed.strip_prefix("assets/").unwrap_or(&trimmed)),
    ];

    for candidate in &candidates {
        if candidate.is_file() {
            let assets_canon = assets.canonicalize().unwrap_or(assets.clone());
            let cand_canon = candidate.canonicalize().unwrap_or(candidate.clone());
            if cand_canon.starts_with(&assets_canon) {
                return Ok(candidate.clone());
            }
        }
    }

    Err(format!("asset not found: {trimmed}"))
}

fn mime_for_path(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()).unwrap_or("") {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

fn title_from_markdown(content: &str, fallback: &str) -> String {
    if let Some(line) = content.lines().find(|l| l.starts_with("title:")) {
        let t = line.trim_start_matches("title:").trim().trim_matches('"');
        if !t.is_empty() {
            return t.to_string();
        }
    }
    for line in content.lines() {
        if let Some(h) = line.strip_prefix("# ") {
            return h.trim().to_string();
        }
    }
    fallback.to_string()
}

fn collect_wiki_files(dir: &Path, base: &Path, out: &mut Vec<WikiEntry>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("read_dir {}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_wiki_files(&path, base, out)?;
        } else if path.extension().is_some_and(|e| e == "md") {
            let rel = path
                .strip_prefix(base)
                .map_err(|e| e.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            let content = fs::read_to_string(&path).unwrap_or_default();
            let fallback = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("untitled");
            out.push(WikiEntry {
                path: rel,
                title: title_from_markdown(&content, fallback),
            });
        }
    }
    Ok(())
}

#[tauri::command]
fn list_wiki_entries(app: AppHandle) -> Result<Vec<WikiEntry>, String> {
    let wiki = vault_wiki_dir(&app)?;
    let mut entries = Vec::new();
    collect_wiki_files(&wiki, &wiki, &mut entries)?;
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(entries)
}

#[tauri::command]
fn read_wiki_file(app: AppHandle, relative_path: String) -> Result<String, String> {
    if relative_path.contains("..") {
        return Err("invalid path".into());
    }
    let wiki = vault_wiki_dir(&app)?;
    let file = wiki.join(&relative_path);
    let wiki_canon = wiki.canonicalize().unwrap_or(wiki);
    let file_canon = file.canonicalize().unwrap_or(file);
    if !file_canon.starts_with(&wiki_canon) {
        return Err("path outside vault".into());
    }
    fs::read_to_string(&file_canon).map_err(|e| format!("read {}: {e}", file_canon.display()))
}

#[tauri::command]
fn get_vault_info(app: AppHandle) -> Result<String, String> {
    let id = active_pack_id(&app);
    let root = pack_root(&app)?;
    Ok(format!("[{id}] {}", root.display()))
}

#[tauri::command]
fn examflow_url() -> String {
    EXAMFLOW_URL.to_string()
}

#[tauri::command]
fn read_asset_data_url(app: AppHandle, relative_path: String) -> Result<String, String> {
    use base64::{engine::general_purpose::STANDARD, Engine as _};

    let file = resolve_asset_path(&app, &relative_path)?;
    let bytes = fs::read(&file).map_err(|e| format!("read {}: {e}", file.display()))?;
    let mime = mime_for_path(&file);
    Ok(format!("data:{mime};base64,{}", STANDARD.encode(bytes)))
}

#[tauri::command]
fn get_assets_dir(app: AppHandle) -> Result<String, String> {
    vault_assets_dir(&app).map(|p| p.display().to_string())
}

// ---- pack commands ---------------------------------------------------------

#[derive(Serialize)]
struct PackInfo {
    id: String,
    name: String,
    active: bool,
}

/// อ่านชื่อวิชาจาก pack.json (fallback = id)
fn read_pack_name(root: &Path, fallback: &str) -> String {
    let raw = match fs::read_to_string(root.join("pack.json")) {
        Ok(r) => r,
        Err(_) => return fallback.to_string(),
    };
    serde_json::from_str::<serde_json::Value>(&raw)
        .ok()
        .as_ref()
        .and_then(|v| v.get("name"))
        .and_then(|n| n.as_str())
        .filter(|n| !n.is_empty())
        .map(|n| n.to_string())
        .unwrap_or_else(|| fallback.to_string())
}

#[tauri::command]
fn list_packs(app: AppHandle) -> Result<Vec<PackInfo>, String> {
    let active = active_pack_id(&app);
    let mut out: Vec<PackInfo> = Vec::new();

    // วิชา built-in (botany) — แสดงเสมอถ้ามี root
    if builtin_pack_root(&app).is_some() {
        out.push(PackInfo {
            id: BUILTIN_PACK_ID.to_string(),
            name: BUILTIN_PACK_NAME.to_string(),
            active: active == BUILTIN_PACK_ID,
        });
    }

    // วิชาที่ผู้ใช้โหลดมาใน packs/
    if let Ok(base) = packs_base_dir(&app) {
        if let Ok(entries) = fs::read_dir(&base) {
            for entry in entries.flatten() {
                let root = entry.path();
                if !root.is_dir() {
                    continue;
                }
                let id = match entry.file_name().to_str() {
                    Some(s) => s.to_string(),
                    None => continue,
                };
                // กันซ้ำ built-in และข้ามแพ็กที่ยังไม่สมบูรณ์
                if id == BUILTIN_PACK_ID || !root.join("wiki").join("index.md").exists() {
                    continue;
                }
                let name = read_pack_name(&root, &id);
                out.push(PackInfo {
                    active: active == id,
                    id,
                    name,
                });
            }
        }
    }

    out.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(out)
}

#[tauri::command]
fn get_active_pack(app: AppHandle) -> String {
    active_pack_id(&app)
}

#[tauri::command]
async fn set_active_pack(app: AppHandle, id: String) -> Result<(), String> {
    validate_pack_id(&id)?;
    // resolve = existence check ในตัว
    let root = if id == BUILTIN_PACK_ID {
        builtin_pack_root(&app).ok_or_else(|| format!("ไม่พบวิชา: {id}"))?
    } else {
        installed_pack_root(&app, &id).ok_or_else(|| format!("ไม่พบวิชา: {id}"))?
    };
    write_active_pack(&app, &id)?;
    set_cached_pack(&app, id, root);

    // โหลดดัชนีวิชาใหม่นอก main thread (parse embeddings.json อาจใหญ่)
    let app2 = app.clone();
    tauri::async_runtime::spawn_blocking(move || {
        let state = app2.state::<ai::AiState>();
        ai::load_index(&app2, &state);
    })
    .await
    .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ai_state = ai::AiState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ai_state)
        .manage(PackState::default())
        .setup(|app| {
            let state = app.state::<ai::AiState>();
            ai::load_index(app.handle(), &state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            list_wiki_entries,
            read_wiki_file,
            get_vault_info,
            examflow_url,
            read_asset_data_url,
            get_assets_dir,
            list_packs,
            get_active_pack,
            set_active_pack,
            set_api_key,
            get_api_key_status,
            ai_chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
