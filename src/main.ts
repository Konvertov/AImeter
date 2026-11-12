import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { duration, lang, setLang, t } from "./i18n";

interface Meter {
  label: string;
  used_percent: number | null;
  detail: string | null;
  resets_at: number | null;
  primary: boolean;
}

interface Snapshot {
  id: string;
  name: string;
  state: "ok" | "not_found" | "expired" | "needs_key" | "rate_limited" | "error";
  message: string | null;
  plan: string | null;
  meters: Meter[];
  fetched_at: number;
  key_based: boolean;
  has_key: boolean;
  key_url: string | null;
}

const root = document.getElementById("app")!;
let snapshots: Snapshot[] = [];
let loaded = false;
let view: "main" | "settings" = "main";
let refreshing = false;

/** Small helper: build an element with classes, text and children. */
function el(tag: string, cls = "", text = "", children: Node[] = []): HTMLElement {
  const node = document.createElement(tag);
  if (cls) node.className = cls;
  if (text) node.textContent = text;
  for (const child of children) node.appendChild(child);
  return node;
}

function button(label: string, cls: string, onClick: () => void): HTMLButtonElement {
  const node = el("button", cls, label) as HTMLButtonElement;
  node.addEventListener("click", onClick);
  return node;
}

function level(percent: number): string {
  if (percent >= 90) return "high";
  if (percent >= 70) return "mid";
  return "low";
}

function meterRow(meter: Meter): HTMLElement {
  const row = el("div", "meter");
  const right =
    meter.used_percent !== null ? `${Math.round(meter.used_percent)}%` : (meter.detail ?? "");
  row.appendChild(
    el("div", "meter-head", "", [el("span", "meter-label", t(meter.label)), el("span", "meter-value", right)]),
  );
  if (meter.used_percent !== null) {
    const fill = el("div", `bar-fill ${level(meter.used_percent)}`);
    fill.style.width = `${Math.max(2, Math.min(100, meter.used_percent))}%`;
    row.appendChild(el("div", "bar", "", [fill]));
    const notes: string[] = [];
    if (meter.detail) notes.push(meter.detail);
    if (meter.resets_at !== null) {
      const left = meter.resets_at - Date.now();
      notes.push(left > 60000 ? `${t("resets_in")} ${duration(left)}` : t("resets_now"));
    }
    if (notes.length) row.appendChild(el("div", "meter-note", notes.join(" · ")));
  }
  return row;
}

function card(snap: Snapshot): HTMLElement {
  const head = el("div", "card-head", "", [el("span", "card-name", snap.name)]);
  if (snap.plan) head.appendChild(el("span", "plan", snap.plan));
  const node = el("section", "card", "", [head]);
  if (snap.state !== "ok") {
    const text =
      snap.state === "error" && snap.message
        ? `${t("state_error")} (${snap.message})`
        : t(`state_${snap.state}`);
    node.appendChild(el("div", "notice", text));
  }
  for (const meter of snap.meters) node.appendChild(meterRow(meter));
  return node;
}

function mainView(): HTMLElement {
  const shown = snapshots.filter((s) => s.state !== "not_found" && !(s.state === "needs_key"));
  const list = el("div", "list");
  if (!loaded) {
    list.appendChild(el("div", "empty", "", [el("p", "empty-title", t("loading"))]));
  } else if (shown.length === 0) {
    list.appendChild(
      el("div", "empty", "", [el("p", "empty-title", t("empty_title")), el("p", "empty-hint", t("empty_hint"))]),
    );
  } else {
    for (const snap of shown) list.appendChild(card(snap));
  }

  const newest = Math.max(0, ...shown.map((s) => s.fetched_at));
  const stamp = newest
    ? new Date(newest).toLocaleTimeString(lang() === "ru" ? "ru-RU" : "en-US", { hour: "2-digit", minute: "2-digit" })
    : t("never");
  const refreshButton = button(refreshing ? "…" : t("refresh"), "ghost", () => void refresh());
  refreshButton.disabled = refreshing;
  const footer = el("footer", "footer", "", [
    el("span", "stamp", `${t("updated")}: ${stamp}`),
    el("span", "footer-actions", "", [
      refreshButton,
      button(t("settings"), "ghost", () => {
        view = "settings";
        render();
      }),
    ]),
  ]);
  return el("div", "screen", "", [el("header", "top", "", [el("h1", "", "AImeter")]), list, footer]);
}

function keyRow(snap: Snapshot): HTMLElement {
  const input = el("input", "key-input") as HTMLInputElement;
  input.type = "password";
  input.placeholder = snap.has_key ? t("key_saved") : t("paste_key");
  input.autocomplete = "off";
  input.spellcheck = false;
  const save = async (value: string) => {
    try {
      await invoke("set_api_key", { id: snap.id, key: value });
    } catch (error) {
      input.value = "";
      input.placeholder = String(error);
    }
  };
  const actions = el("div", "key-actions", "", [button(t("save"), "solid", () => void save(input.value))]);
  if (snap.has_key) actions.appendChild(button(t("remove"), "ghost", () => void save("")));
  if (snap.key_url) {
    const url = snap.key_url;
    actions.appendChild(button(t("get_key"), "link", () => void invoke("open_link", { url })));
  }
  return el("div", "key-row", "", [el("div", "key-name", snap.name), input, actions]);
}

function settingsView(): HTMLElement {
  const language = el("div", "segmented");
  for (const [code, label] of [["en", "English"], ["ru", "Русский"]] as const) {
    language.appendChild(
      button(label, lang() === code ? "seg active" : "seg", () => {
        setLang(code);
        render();
      }),
    );
  }

  const body = el("div", "list");
  body.appendChild(el("section", "card", "", [el("h2", "", t("language")), language]));

  const keys = el("section", "card", "", [el("h2", "", t("api_keys")), el("p", "hint", t("api_keys_hint"))]);
  for (const snap of snapshots.filter((s) => s.key_based)) keys.appendChild(keyRow(snap));
  body.appendChild(keys);

  const auto = snapshots.filter((s) => !s.key_based);
  const found = auto.filter((s) => s.state !== "not_found");
  const missing = auto.filter((s) => s.state === "not_found");
  if (found.length) {
    body.appendChild(
      el("section", "card", "", [el("h2", "", t("detected")), el("p", "names", found.map((s) => s.name).join(", "))]),
    );
  }
  if (missing.length) {
    body.appendChild(
      el("section", "card", "", [
        el("h2", "", t("not_detected")),
        el("p", "names", missing.map((s) => s.name).join(", ")),
        el("p", "hint", t("not_detected_hint")),
      ]),
    );
  }

  const top = el("header", "top", "", [
    button(`‹ ${t("back")}`, "ghost", () => {
      view = "main";
      render();
    }),
    el("h1", "", t("settings")),
  ]);
  const footer = el("footer", "footer", "", [
    el("span", "stamp", "AImeter 0.1.0"),
    button(t("quit"), "ghost", () => void invoke("quit")),
  ]);
  return el("div", "screen", "", [top, body, footer]);
}

function render(): void {
  // Do not redraw under the user's fingers while a key is being typed.
  const active = document.activeElement;
  if (active instanceof HTMLInputElement && active.value !== "") return;
  document.documentElement.lang = lang();
  root.replaceChildren(view === "main" ? mainView() : settingsView());
}

async function refresh(): Promise<void> {
  refreshing = true;
  render();
  try {
    await invoke("refresh_now");
  } finally {
    refreshing = false;
    render();
  }
}

function accept(next: Snapshot[]): void {
  snapshots = next;
  if (next.length) loaded = true;
  render();
}

void listen<Snapshot[]>("snapshots", (event) => accept(event.payload));
void invoke<Snapshot[]>("get_snapshots").then(accept);
// Countdown texts tick on their own.
setInterval(() => {
  if (view === "main") render();
}, 30000);
render();
