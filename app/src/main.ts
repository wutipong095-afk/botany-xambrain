import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import {
  buildLinkIndex,
  renderMarkdown,
  resolveWikiLink,
  stripLeadingHeading,
  type WikiEntry,
} from "./markdown";
import { clearChat, mountChat, setChatNavigate } from "./chat";

interface PackInfo {
  id: string;
  name: string;
  active: boolean;
}

let wikiEntries: WikiEntry[] = [];
let activePackId = "botany";
let switchGeneration = 0;

const wikiListEl = document.querySelector("#wiki-list") as HTMLUListElement;
const packSelectEl = document.querySelector("#pack-select") as HTMLSelectElement;
const vaultPathEl = document.querySelector("#vault-path") as HTMLParagraphElement;
const welcomeEl = document.querySelector("#welcome") as HTMLDivElement;
const articleEl = document.querySelector("#article") as HTMLElement;
const articleTitleEl = document.querySelector("#article-title") as HTMLHeadingElement;
const articleBodyEl = document.querySelector("#article-body") as HTMLDivElement;

async function openExamFlow() {
  const url = await invoke<string>("examflow_url");
  await openUrl(url);
}

function setActiveSidebar(path: string) {
  document.querySelectorAll(".wiki-list li").forEach((li) => li.classList.remove("active"));
  document.querySelector(`[data-path="${CSS.escape(path)}"]`)?.classList.add("active");
}

async function hydrateImages(container: HTMLElement) {
  const imgs = container.querySelectorAll<HTMLImageElement>("img.wiki-image.pending");
  await Promise.all(
    [...imgs].map(async (img) => {
      const path = img.getAttribute("data-asset");
      if (!path) return;
      const width = img.getAttribute("data-width");
      try {
        const dataUrl = await invoke<string>("read_asset_data_url", { relativePath: path });
        img.src = dataUrl;
        img.classList.remove("pending");
        if (width) img.style.maxWidth = `${width}px`;
      } catch {
        img.classList.add("wiki-image-missing");
        img.alt = `ไม่พบรูป: ${path}`;
        const cap = img.parentElement?.querySelector(".wiki-figure-caption");
        if (cap) cap.textContent = `ไม่พบรูป: ${path}`;
      }
    })
  );
}

async function loadWikiFile(entry: WikiEntry) {
  const raw = await invoke<string>("read_wiki_file", { relativePath: entry.path });
  welcomeEl.hidden = true;
  articleEl.hidden = false;
  articleTitleEl.textContent = entry.title;

  let html = renderMarkdown(raw);
  html = stripLeadingHeading(html, entry.title);
  articleBodyEl.innerHTML = html;

  articleBodyEl.querySelectorAll('a[href^="http"]').forEach((a) => {
    a.setAttribute("target", "_blank");
    a.setAttribute("rel", "noopener noreferrer");
  });

  await hydrateImages(articleBodyEl);

  setActiveSidebar(entry.path);
}

async function navigateWikiLink(target: string) {
  const entry = resolveWikiLink(target);
  if (entry) {
    await loadWikiFile(entry);
    return;
  }
  articleBodyEl.insertAdjacentHTML(
    "beforeend",
    `<p class="link-error">ไม่พบบทเรียน: ${target}</p>`
  );
}

function onArticleClick(e: MouseEvent) {
  const el = (e.target as HTMLElement).closest("a.wikilink");
  if (!el) return;
  e.preventDefault();
  const target = el.getAttribute("data-wiki");
  if (target) void navigateWikiLink(target);
}

async function updateVaultPath() {
  try {
    vaultPathEl.textContent = await invoke<string>("get_vault_info");
  } catch {
    vaultPathEl.textContent = "ไม่พบ vault wiki/";
  }
}

function renderSidebar() {
  wikiListEl.innerHTML = "";
  for (const entry of wikiEntries) {
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
}

/** โหลดบทเรียนของวิชาที่ active เข้าแถบข้าง + เปิดหน้า index */
async function loadVault() {
  await updateVaultPath();
  wikiEntries = await invoke<WikiEntry[]>("list_wiki_entries");
  buildLinkIndex(wikiEntries);
  renderSidebar();

  const indexEntry = wikiEntries.find((e) => e.path === "index.md");
  if (indexEntry) {
    await loadWikiFile(indexEntry);
  } else {
    // วิชาที่ไม่มี index.md → กลับหน้าต้อนรับ
    articleEl.hidden = true;
    welcomeEl.hidden = false;
  }
}

/** เติมรายชื่อวิชาลง dropdown (ซ่อนถ้ามีวิชาเดียว) */
async function loadPacks() {
  const packs = await invoke<PackInfo[]>("list_packs");
  packSelectEl.innerHTML = "";
  for (const p of packs) {
    const opt = document.createElement("option");
    opt.value = p.id;
    opt.textContent = p.name;
    if (p.active) {
      opt.selected = true;
      activePackId = p.id;
    }
    packSelectEl.appendChild(opt);
  }
  packSelectEl.hidden = packs.length <= 1;
}

async function switchPack(id: string) {
  if (id === activePackId) return;

  const prev = activePackId;
  const gen = ++switchGeneration;
  packSelectEl.disabled = true;

  try {
    await invoke("set_active_pack", { id });
    if (gen !== switchGeneration) return;

    clearChat();
    activePackId = id;
    await loadVault();
    if (gen !== switchGeneration) return;
  } catch (e) {
    if (gen !== switchGeneration) return;
    packSelectEl.value = prev;
    vaultPathEl.textContent = `สลับวิชาไม่สำเร็จ: ${String(e)}`;
  } finally {
    if (gen === switchGeneration) {
      packSelectEl.disabled = false;
    }
  }
}

async function init() {
  document.querySelector("#btn-examflow")?.addEventListener("click", () => void openExamFlow());
  document.querySelector("#link-examflow")?.addEventListener("click", (e) => {
    e.preventDefault();
    void openExamFlow();
  });

  articleBodyEl.addEventListener("click", onArticleClick);

  // mount AI chat panel
  const chatPanelEl = document.getElementById("chat-panel");
  if (chatPanelEl) {
    mountChat(chatPanelEl);
    setChatNavigate(async (file: string) => {
      const entry = wikiEntries.find((e) => e.path === file);
      if (entry) await loadWikiFile(entry);
    });
  }

  packSelectEl.addEventListener("change", () => {
    void switchPack(packSelectEl.value);
  });

  await loadPacks();
  await loadVault();
}

window.addEventListener("DOMContentLoaded", () => {
  void init().catch((err) => {
    vaultPathEl.textContent = `โหลด vault ไม่สำเร็จ: ${String(err)}`;
  });
});
