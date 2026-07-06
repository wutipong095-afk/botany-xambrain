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

fn dev_vault_wiki() -> Option<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dev_wiki = manifest.join("..").join("..").join("wiki");
    if dev_wiki.join("index.md").exists() {
        Some(dev_wiki.canonicalize().unwrap_or(dev_wiki))
    } else {
        None
    }
}

fn bundled_vault_wiki(app: &AppHandle) -> Option<PathBuf> {
    app.path()
        .resolve("vault/wiki", BaseDirectory::Resource)
        .ok()
        .filter(|p| p.join("index.md").exists())
}

fn local_resources_vault_wiki() -> Option<PathBuf> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let local = manifest.join("resources").join("vault").join("wiki");
    if local.join("index.md").exists() {
        Some(local.canonicalize().unwrap_or(local))
    } else {
        None
    }
}

fn vault_wiki_dir(app: &AppHandle) -> Result<PathBuf, String> {
    #[cfg(debug_assertions)]
    if let Some(p) = dev_vault_wiki() {
        return Ok(p);
    }

    if let Some(p) = bundled_vault_wiki(app) {
        return Ok(p);
    }

    if let Some(p) = dev_vault_wiki() {
        return Ok(p);
    }

    if let Some(p) = local_resources_vault_wiki() {
        return Ok(p);
    }

    Err("vault wiki/ not found (bundle vault/wiki or dev ../../wiki)".into())
}

fn vault_root_dir(app: &AppHandle) -> Result<PathBuf, String> {
    vault_wiki_dir(app)?
        .parent()
        .ok_or_else(|| "vault root has no parent".to_string())
        .map(|p| p.to_path_buf())
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
    let wiki = vault_wiki_dir(&app)?;
    let mode = if bundled_vault_wiki(&app).is_some_and(|b| b == wiki) {
        "bundled"
    } else {
        "dev"
    };
    Ok(format!("[{mode}] {}", wiki.display()))
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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let ai_state = ai::AiState::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ai_state)
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
            set_api_key,
            get_api_key_status,
            ai_chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
