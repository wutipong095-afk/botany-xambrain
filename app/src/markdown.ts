import { marked } from "marked";

export interface WikiEntry {
  path: string;
  title: string;
}

let linkIndex = new Map<string, WikiEntry>();

/** สร้างดัชนี [[wikilink]] → ไฟล์ */
export function buildLinkIndex(entries: WikiEntry[]): void {
  linkIndex = new Map();
  for (const entry of entries) {
    const stem = entry.path.replace(/\.md$/i, "");
    linkIndex.set(stem.toLowerCase(), entry);
    const base = stem.split("/").pop()!;
    if (!linkIndex.has(base.toLowerCase())) {
      linkIndex.set(base.toLowerCase(), entry);
    }
    linkIndex.set(entry.title.toLowerCase(), entry);
  }
}

export function resolveWikiLink(target: string): WikiEntry | undefined {
  const key = target.trim().replace(/\.md$/i, "").toLowerCase();
  if (linkIndex.has(key)) return linkIndex.get(key);
  return [...linkIndex.values()].find((e) =>
    e.path.replace(/\.md$/i, "").toLowerCase().endsWith(`/${key}`)
  );
}

export function stripFrontmatter(raw: string): string {
  if (raw.startsWith("---")) {
    const end = raw.indexOf("\n---", 3);
    if (end !== -1) return raw.slice(end + 4).trimStart();
  }
  return raw;
}

function escapeAttr(value: string): string {
  return value
    .replace(/&/g, "&amp;")
    .replace(/"/g, "&quot;")
    .replace(/</g, "&lt;");
}

function wikilinksToMarkdown(text: string): string {
  // ![[path/to.png|500]] — รูปจาก assets/ (width หลัง | เป็นตัวเลข)
  text = text.replace(/!\[\[([^\]|]+)(?:\|([^\]]+))?\]\]/g, (_, target, option) => {
    const t = target.trim();
    const opt = option?.trim() ?? "";
    const width = /^\d+$/.test(opt) ? opt : null;
    const alt = t.split("/").pop() ?? t;
    const widthAttr = width ? ` data-width="${width}"` : "";
    return `<figure class="wiki-figure"><img class="wiki-image pending" data-asset="${escapeAttr(t)}"${widthAttr} alt="${escapeAttr(alt)}"><figcaption class="wiki-figure-caption">${escapeAttr(alt)}</figcaption></figure>`;
  });
  // [[target|label]] หรือ [[target]]
  return text.replace(/\[\[([^\]|]+)(?:\|([^\]]+))?\]\]/g, (_, target, display) => {
    const label = (display ?? target).trim();
    const t = target.trim();
    const resolved = resolveWikiLink(t);
    const cls = resolved ? "wikilink" : "wikilink wikilink-missing";
    return `<a href="#" class="${cls}" data-wiki="${escapeAttr(t)}">${label}</a>`;
  });
}

marked.use({
  gfm: true,
  breaks: false,
});

export function renderMarkdown(raw: string): string {
  const body = stripFrontmatter(raw);
  const withLinks = wikilinksToMarkdown(body);
  return marked.parse(withLinks, { async: false }) as string;
}

export function stripLeadingHeading(html: string, title: string): string {
  // ถ้าเนื้อหาเริ่มด้วย h1 ซ้ำกับ title ให้ตัดออก
  const tmp = document.createElement("div");
  tmp.innerHTML = html;
  const h1 = tmp.querySelector("h1");
  if (h1 && h1.textContent?.trim() === title.trim()) {
    h1.remove();
  }
  return tmp.innerHTML;
}
