use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

const EXAMFLOW_URL: &str = "https://test.xambrain.com";

#[derive(Serialize)]
struct WikiEntry {
    path: String,
    title: String,
}

fn vault_wiki_dir() -> Result<PathBuf, String> {
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let dev_wiki = manifest.join("..").join("..").join("wiki");
    if dev_wiki.join("index.md").exists() {
        return dev_wiki
            .canonicalize()
            .map_err(|e| format!("cannot resolve dev wiki path: {e}"));
    }

    let bundled = manifest.join("resources").join("vault").join("wiki");
    if bundled.join("index.md").exists() {
        return bundled
            .canonicalize()
            .map_err(|e| format!("cannot resolve bundled wiki path: {e}"));
    }

    Err("vault wiki/ not found (dev: ../../wiki or bundle: resources/vault/wiki)".into())
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
fn list_wiki_entries() -> Result<Vec<WikiEntry>, String> {
    let wiki = vault_wiki_dir()?;
    let mut entries = Vec::new();
    collect_wiki_files(&wiki, &wiki, &mut entries)?;
    entries.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(entries)
}

#[tauri::command]
fn read_wiki_file(relative_path: String) -> Result<String, String> {
    if relative_path.contains("..") {
        return Err("invalid path".into());
    }
    let wiki = vault_wiki_dir()?;
    let file = wiki.join(&relative_path);
    if !file.starts_with(&wiki) {
        return Err("path outside vault".into());
    }
    fs::read_to_string(&file).map_err(|e| format!("read {}: {e}", file.display()))
}

#[tauri::command]
fn get_vault_info() -> Result<String, String> {
    vault_wiki_dir().map(|p| p.display().to_string())
}

#[tauri::command]
fn examflow_url() -> String {
    EXAMFLOW_URL.to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            list_wiki_entries,
            read_wiki_file,
            get_vault_info,
            examflow_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
