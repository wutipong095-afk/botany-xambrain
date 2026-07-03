import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";

interface WikiEntry {
  path: string;
  title: string;
}

const wikiListEl = document.querySelector("#wiki-list") as HTMLUListElement;
const vaultPathEl = document.querySelector("#vault-path") as HTMLParagraphElement;
const welcomeEl = document.querySelector("#welcome") as HTMLDivElement;
const articleEl = document.querySelector("#article") as HTMLElement;
const articleTitleEl = document.querySelector("#article-title") as HTMLHeadingElement;
const articleBodyEl = document.querySelector("#article-body") as HTMLPreElement;

function stripFrontmatter(raw: string): string {
  if (raw.startsWith("---")) {
    const end = raw.indexOf("\n---", 3);
    if (end !== -1) {
      return raw.slice(end + 4).trimStart();
    }
  }
  return raw;
}

async function openExamFlow() {
  const url = await invoke<string>("examflow_url");
  await openUrl(url);
}

async function loadWikiFile(entry: WikiEntry) {
  const raw = await invoke<string>("read_wiki_file", { relativePath: entry.path });
  welcomeEl.hidden = true;
  articleEl.hidden = false;
  articleTitleEl.textContent = entry.title;
  articleBodyEl.textContent = stripFrontmatter(raw);

  document.querySelectorAll(".wiki-list li").forEach((li) => li.classList.remove("active"));
  document.querySelector(`[data-path="${CSS.escape(entry.path)}"]`)?.classList.add("active");
}

async function init() {
  document.querySelector("#btn-examflow")?.addEventListener("click", () => void openExamFlow());
  document.querySelector("#link-examflow")?.addEventListener("click", (e) => {
    e.preventDefault();
    void openExamFlow();
  });

  try {
    const vaultPath = await invoke<string>("get_vault_info");
    vaultPathEl.textContent = vaultPath;
  } catch {
    vaultPathEl.textContent = "ไม่พบ vault wiki/";
  }

  const entries = await invoke<WikiEntry[]>("list_wiki_entries");
  wikiListEl.innerHTML = "";

  for (const entry of entries) {
    const li = document.createElement("li");
    li.dataset.path = entry.path;
    const btn = document.createElement("button");
    btn.type = "button";
    btn.textContent = entry.title;
    btn.title = entry.path;
    btn.addEventListener("click", () => void loadWikiFile(entry));
    li.appendChild(btn);
    wikiListEl.appendChild(li);
  }

  const indexEntry = entries.find((e) => e.path === "index.md");
  if (indexEntry) {
    await loadWikiFile(indexEntry);
  }
}

window.addEventListener("DOMContentLoaded", () => {
  void init().catch((err) => {
    vaultPathEl.textContent = `โหลด vault ไม่สำเร็จ: ${String(err)}`;
  });
});
