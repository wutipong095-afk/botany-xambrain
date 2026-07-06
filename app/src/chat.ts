import { invoke } from "@tauri-apps/api/core";

export interface Citation {
  title: string;
  file: string;
  heading: string;
}

interface ChatResponse {
  answer: string;
  citations: Citation[];
}

// ---- state ----------------------------------------------------------------

let onNavigate: ((file: string) => void) | null = null;

/** ลงทะเบียน callback สำหรับเปิดบทเรียนเมื่อคลิก citation */
export function setChatNavigate(fn: (file: string) => void) {
  onNavigate = fn;
}

// ---- DOM refs (ถูกสร้างใน mountChat) ------------------------------------

let chatPanel: HTMLElement | null = null;
let messagesEl: HTMLElement | null = null;
let inputEl: HTMLInputElement | null = null;
let sendBtn: HTMLButtonElement | null = null;
let keyModal: HTMLElement | null = null;
let keyInput: HTMLInputElement | null = null;
let keyBadge: HTMLElement | null = null;

// ---- mounting --------------------------------------------------------------

export function mountChat(container: HTMLElement) {
  // Chat panel (ไม่รวม modal — modal mount ที่ body แยก)
  container.innerHTML = `
    <div class="chat-header">
      <span class="chat-title">ถามครู AI</span>
      <button type="button" class="chat-key-btn" id="chat-key-btn" title="ตั้งค่า API key">🔑</button>
    </div>
    <div class="chat-key-badge" id="chat-key-badge">ยังไม่มี key</div>
    <form class="chat-form" id="chat-form" autocomplete="off">
      <input
        type="text"
        class="chat-input"
        id="chat-input"
        placeholder="ถามเกี่ยวกับพฤกษศาสตร์..."
        maxlength="400"
      />
      <button type="submit" class="btn primary chat-send" id="chat-send">ส่ง</button>
    </form>
    <div class="chat-messages" id="chat-messages" aria-live="polite"></div>
  `;

  // Modal mount ที่ body — ป้องกัน overlay ทับ form
  const modalEl = document.createElement("div");
  modalEl.innerHTML = `
    <div class="key-modal-overlay" id="key-modal" hidden>
      <div class="key-modal-box" role="dialog" aria-modal="true" aria-labelledby="key-modal-title">
        <h3 id="key-modal-title">ตั้งค่า Gemini API Key</h3>
        <p class="key-modal-note">
          Key เก็บในโฟลเดอร์ config ของแอป — ไม่ถูกส่งออนไลน์นอกจาก Gemini API
        </p>
        <input
          type="password"
          class="key-input"
          id="key-input"
          placeholder="AIza..."
          autocomplete="off"
        />
        <div class="key-modal-actions">
          <button type="button" class="btn primary" id="key-save-btn">บันทึก</button>
          <button type="button" class="btn" id="key-cancel-btn">ยกเลิก</button>
        </div>
      </div>
    </div>
  `;
  document.body.appendChild(modalEl.firstElementChild!);

  chatPanel = container;
  messagesEl = document.getElementById("chat-messages");
  inputEl = document.getElementById("chat-input") as HTMLInputElement;
  sendBtn = document.getElementById("chat-send") as HTMLButtonElement;
  keyModal = document.getElementById("key-modal");
  keyInput = document.getElementById("key-input") as HTMLInputElement;
  keyBadge = document.getElementById("chat-key-badge");

  // events
  document.getElementById("chat-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    void handleSend();
  });

  document.getElementById("chat-key-btn")?.addEventListener("click", openKeyModal);
  document.getElementById("key-save-btn")?.addEventListener("click", () => void saveKey());
  document.getElementById("key-cancel-btn")?.addEventListener("click", closeKeyModal);
  keyModal?.addEventListener("click", (e) => {
    if (e.target === keyModal) closeKeyModal();
  });

  void refreshKeyBadge();
}

// ---- key modal ------------------------------------------------------------

function openKeyModal() {
  if (!keyModal) return;
  keyModal.hidden = false;
  keyInput?.focus();
}

function closeKeyModal() {
  if (!keyModal) return;
  keyModal.hidden = true;
  if (keyInput) keyInput.value = "";
}

async function saveKey() {
  const key = keyInput?.value.trim() ?? "";
  if (!key) return;
  try {
    await invoke("set_api_key", { key });
    closeKeyModal();
    await refreshKeyBadge();
    appendSystemMsg("บันทึก API key เรียบร้อย");
  } catch (e) {
    appendSystemMsg(`บันทึก key ไม่สำเร็จ: ${String(e)}`);
  }
}

async function refreshKeyBadge() {
  if (!keyBadge) return;
  try {
    const ok = await invoke<boolean>("get_api_key_status");
    keyBadge.textContent = ok ? "มี key แล้ว ✓" : "ยังไม่มี key";
    keyBadge.className = `chat-key-badge ${ok ? "key-ok" : "key-missing"}`;
  } catch {
    keyBadge.textContent = "ไม่ทราบสถานะ key";
  }
}

// ---- chat send -------------------------------------------------------------

async function handleSend() {
  const question = inputEl?.value.trim() ?? "";
  if (!question || !messagesEl) return;

  if (inputEl) inputEl.value = "";
  setFormDisabled(true);

  appendUserMsg(question);
  const loadingEl = appendSystemMsg("กำลังคิด…", true);

  try {
    const res = await invoke<ChatResponse>("ai_chat", { question });
    loadingEl.remove();
    appendBotMsg(res.answer, res.citations);
  } catch (e) {
    loadingEl.remove();
    const errStr = String(e);
    console.error("[ai_chat]", e);
    appendSystemMsg(`ข้อผิดพลาด: ${errStr}`);
  } finally {
    setFormDisabled(false);
    inputEl?.focus();
  }
}

function setFormDisabled(disabled: boolean) {
  if (inputEl) inputEl.disabled = disabled;
  if (sendBtn) sendBtn.disabled = disabled;
}

// ---- message builders -----------------------------------------------------

// ข้อความใหม่อยู่บนสุด (ใต้ช่องพิมพ์) → เลื่อนขึ้นบนสุดเพื่อโชว์อันล่าสุด
function scrollToNewest() {
  if (messagesEl) messagesEl.scrollTop = 0;
}

function appendUserMsg(text: string): HTMLElement {
  const el = document.createElement("div");
  el.className = "chat-msg user";
  el.textContent = text;
  messagesEl?.prepend(el);
  scrollToNewest();
  return el;
}

function appendSystemMsg(text: string, loading = false): HTMLElement {
  const el = document.createElement("div");
  el.className = `chat-msg system${loading ? " loading" : ""}`;
  el.textContent = text;
  messagesEl?.prepend(el);
  scrollToNewest();
  return el;
}

function appendBotMsg(text: string, citations: Citation[]): HTMLElement {
  const el = document.createElement("div");
  el.className = "chat-msg bot";

  const body = document.createElement("div");
  body.className = "chat-answer";
  // แสดง markdown ขั้นต้น (bold + newline)
  body.innerHTML = basicMarkdown(text);
  el.appendChild(body);

  if (citations.length > 0) {
    const citeRow = document.createElement("div");
    citeRow.className = "chat-citations";
    citeRow.textContent = "อ้างอิง: ";
    citations.forEach((c, i) => {
      const chip = document.createElement("button");
      chip.type = "button";
      chip.className = "cite-chip";
      chip.textContent = c.heading ? `${c.title} › ${c.heading}` : c.title;
      chip.title = c.file;
      chip.addEventListener("click", () => {
        onNavigate?.(c.file);
      });
      citeRow.appendChild(chip);
      if (i < citations.length - 1) {
        citeRow.appendChild(document.createTextNode(" "));
      }
    });
    el.appendChild(citeRow);
  }

  messagesEl?.prepend(el);
  scrollToNewest();
  return el;
}

/** แปลง markdown พื้นฐาน (bold, italic, newline) เป็น HTML */
function basicMarkdown(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>")
    .replace(/\*(.+?)\*/g, "<em>$1</em>")
    .replace(/\n/g, "<br>");
}
