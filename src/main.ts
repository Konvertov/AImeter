import { getVersion } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { author } from "./author";
import { autoUpdate, clock, duration, lang, setAutoUpdate, setClock, setLang, setShowLeft, setTrayService, showLeft, t, timeOfDay, trayService } from "./i18n";
import { logo } from "./logos";

interface Meter {
  label: string;
  used_percent: number | null;
  detail: string | null;
  resets_at: number | null;
  primary: boolean;
  window_secs: number | null;
}

interface Stats {
  today_tokens: number;
  month_tokens: number;
  days: number[];
  top_model: string | null;
  today_cost: number;
  month_cost: number;
  cost_partial: boolean;
}

interface Snapshot {
  id: string;
  name: string;
  state: "ok" | "not_found" | "expired" | "needs_key" | "rate_limited" | "error";
  message: string | null;
  plan: string | null;
  account: string | null;
  meters: Meter[];
  stats: Stats | null;
  fetched_at: number;
  key_based: boolean;
  has_key: boolean;
  key_url: string | null;
  connect: boolean;
}

const root = document.getElementById("app")!;
let snapshots: Snapshot[] = [];
let loaded = false;
let view: "main" | "settings" = "main";
let refreshing = false;
let version = "";
/** True while the "really quit?" question is on screen. */
let confirmingQuit = false;

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

const ICONS = {
  refresh: '<path d="M20 12a8 8 0 1 1-2.6-5.9"/><path d="M20 4v5h-5"/>',
  settings:
    '<circle cx="12" cy="12" r="6.4"/><circle cx="12" cy="12" r="2.6"/>' +
    '<path d="M12 2.6v3M12 18.4v3M2.6 12h3M18.4 12h3M5.4 5.4l2.1 2.1M16.5 16.5l2.1 2.1M5.4 18.6l2.1-2.1M16.5 7.5l2.1-2.1"/>',
};

/** A button that shows a small line drawing instead of a word; the word stays as a tooltip. */
function iconButton(icon: keyof typeof ICONS, label: string, onClick: () => void): HTMLButtonElement {
  const node = button("", "ghost icon", onClick);
  node.title = label;
  node.setAttribute("aria-label", label);
  node.innerHTML =
    '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" ' +
    `stroke-linecap="round" stroke-linejoin="round" aria-hidden="true">${ICONS[icon]}</svg>`;
  return node;
}

function level(percent: number): string {
  if (percent >= 90) return "high";
  if (percent >= 70) return "mid";
  return "low";
}

/** Where the limit should be by now if it were spent evenly, and what follows from that. */
function pace(meter: Meter): { expected: number; status: string; warning: string | null } | null {
  if (meter.used_percent === null || meter.resets_at === null || !meter.window_secs) return null;
  const length = meter.window_secs * 1000;
  const left = meter.resets_at - Date.now();
  if (left <= 0 || left > length) return null;
  const elapsed = length - left;
  if (elapsed / length < 0.03) return null; // too early to judge
  const expected = (elapsed / length) * 100;
  const used = meter.used_percent;
  const gap = Math.round(expected - used);
  const status =
    gap >= 2 ? `${gap}% ${t("in_reserve")}` : gap <= -2 ? `${-gap}% ${t("over_pace")}` : t("on_pace");
  let warning: string | null = null;
  if (used >= 100) {
    warning = t("limit_reached");
  } else if (used > 0) {
    const untilEmpty = ((100 - used) / used) * elapsed;
    if (untilEmpty < left) warning = `${t("runs_out_in")} ${duration(untilEmpty)}`;
  }
  return { expected, status, warning };
}

function meterRow(meter: Meter): HTMLElement {
  const row = el("div", "meter");
  const left = showLeft();
  let right = meter.detail ?? "";
  if (meter.used_percent !== null) {
    const used = Math.round(meter.used_percent);
    if (!left) right = `${used}%`;
    else right = lang() === "ru" ? `${t("left")} ${100 - used}%` : `${100 - used}% ${t("left")}`;
  }
  row.appendChild(
    el("div", "meter-head", "", [el("span", "meter-label", t(meter.label)), el("span", "meter-value", right)]),
  );
  if (meter.used_percent !== null) {
    // The colour always follows how much is spent; the bar length follows the chosen view.
    const shown = left ? 100 - meter.used_percent : meter.used_percent;
    const fill = el("div", `bar-fill ${level(meter.used_percent)}`);
    fill.style.width = `${Math.max(shown > 0 ? 2 : 0, Math.min(100, shown))}%`;
    const bar = el("div", "bar", "", [fill]);
    const tempo = pace(meter);
    if (tempo) {
      const mark = el("div", "bar-mark");
      mark.style.left = `${Math.min(99, left ? 100 - tempo.expected : tempo.expected)}%`;
      bar.appendChild(mark);
    }
    row.appendChild(bar);

    const leftNotes: string[] = [];
    if (meter.detail) leftNotes.push(meter.detail);
    if (tempo) leftNotes.push(tempo.status);
    let reset = "";
    if (meter.resets_at !== null) {
      const left = meter.resets_at - Date.now();
      reset = left > 60000 ? `${t("resets_in")} ${duration(left)}` : t("resets_now");
    }
    if (leftNotes.length || reset) {
      row.appendChild(
        el("div", "meter-note", "", [el("span", "", leftNotes.join(" · ")), el("span", "", reset)]),
      );
    }
    if (tempo?.warning) row.appendChild(el("div", "meter-warning", tempo.warning));
  }
  return row;
}

function tokens(amount: number): string {
  if (amount >= 1e9) return `${(amount / 1e9).toFixed(1)}B`;
  if (amount >= 1e6) return `${(amount / 1e6).toFixed(amount >= 1e8 ? 0 : 1)}M`;
  if (amount >= 1e3) return `${Math.round(amount / 1e3)}K`;
  return String(Math.round(amount));
}

/** Token counts from local logs: two numbers, a bar per day, the most used model. */
function statsBlock(stats: Stats): HTMLElement {
  const figure = (label: string, value: string) =>
    el("div", "figure", "", [el("div", "figure-label", t(label)), el("div", "figure-value", value)]);
  const chart = el("div", "chart");
  const peak = Math.max(1, ...stats.days);
  stats.days.forEach((value, index) => {
    const column = el("div", index === stats.days.length - 1 ? "column today" : "column");
    column.style.height = `${Math.max(value > 0 ? 4 : 1, (value / peak) * 100)}%`;
    const daysAgo = stats.days.length - 1 - index;
    const day = new Date(Date.now() - daysAgo * 86400000).toLocaleDateString(lang() === "ru" ? "ru-RU" : "en-US", {
      day: "numeric",
      month: "short",
    });
    column.title = `${day}: ${tokens(value)}`;
    chart.appendChild(column);
  });
  const block = el("div", "stats", "", [
    el("div", "figures", "", [
      figure("tokens_today", tokens(stats.today_tokens)),
      figure("tokens_month", tokens(stats.month_tokens)),
    ]),
    chart,
  ]);
  const notes = [stats.top_model ? `${t("top_model")}: ${stats.top_model}` : "", t("from_logs")].filter(Boolean);
  block.appendChild(el("div", "meter-note", notes.join(" · ")));
  return block;
}

function card(snap: Snapshot): HTMLElement {
  const names = el("span", "card-names", "", [el("span", "card-name", snap.name)]);
  if (snap.account) names.appendChild(el("span", "account", snap.account));
  const title = el("span", "card-title", "", [logo(snap.id, snap.name), names]);
  const head = el("div", "card-head", "", [title]);
  if (snap.plan) head.appendChild(el("span", "plan", snap.plan));
  const node = el("section", "card", "", [head]);
  if (snap.state !== "ok") {
    const text =
      snap.state === "error" && snap.message
        ? `${t("state_error")} (${snap.message})`
        : t(snap.state === "expired" && snap.connect ? "state_expired_connect" : `state_${snap.state}`);
    node.appendChild(el("div", "notice", text));
  }
  for (const meter of snap.meters) node.appendChild(meterRow(meter));
  if (snap.stats) node.appendChild(statsBlock(snap.stats));
  return node;
}

/** Telegram logo with the author's name; opens the channel. */
function credit(): HTMLElement | null {
  if (!author.url || !author.name) return null;
  const link = button("", "credit-link", () => void invoke("open_link", { url: author.url }));
  link.title = author.url.replace("https://", "");
  link.append(logo("telegram", "Telegram"), el("span", "", author.name));
  return el("div", "credit", "", [link]);
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
  const stamp = newest ? timeOfDay(newest) : t("never");
  const refreshButton = iconButton("refresh", t("refresh"), () => void refresh());
  refreshButton.disabled = refreshing;
  if (refreshing) refreshButton.classList.add("spinning");
  const made = credit();
  const footer = el("footer", "footer", "", [
    el("span", "stamp", `${t("updated")}: ${stamp}`),
    ...(made ? [made] : []),
    el("span", "footer-actions", "", [
      refreshButton,
      iconButton("settings", t("settings"), () => {
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
  return el("div", "key-row", "", [el("div", "key-name", "", [logo(snap.id, snap.name), el("span", "", snap.name)]), input, actions]);
}

/** A service the user connects by signing in once in a window. */
function connectRow(snap: Snapshot): HTMLElement {
  const actions = el("div", "key-actions", "", [
    button(t("connect"), snap.has_key ? "ghost" : "solid", () => void invoke("connect_service", { id: snap.id })),
  ]);
  if (snap.has_key) {
    actions.appendChild(button(t("disconnect"), "ghost", () => void invoke("disconnect_service", { id: snap.id })));
  }
  const status = el("span", "hint", t(snap.has_key ? "connected" : "not_connected"));
  return el("div", "key-row", "", [
    el("div", "key-name", "", [logo(snap.id, snap.name), el("span", "", snap.name), status]),
    actions,
  ]);
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

  const clocks = el("div", "segmented");
  for (const code of ["12", "24"] as const) {
    clocks.appendChild(
      button(t(`clock_${code}`), clock() === code ? "seg active" : "seg", () => {
        setClock(code);
        render();
      }),
    );
  }
  body.appendChild(el("section", "card", "", [el("h2", "", t("time_format")), clocks]));

  const views = el("div", "segmented");
  for (const left of [false, true]) {
    views.appendChild(
      button(t(left ? "show_left" : "show_used"), showLeft() === left ? "seg active" : "seg", () => {
        setShowLeft(left);
        void invoke("set_show_left", { left });
        render();
      }),
    );
  }
  body.appendChild(el("section", "card", "", [el("h2", "", t("show_as")), views]));

  const picker = el("select", "picker") as HTMLSelectElement;
  const option = (value: string, label: string) => {
    const node = el("option", "", label) as HTMLOptionElement;
    node.value = value;
    node.selected = trayService() === value;
    picker.appendChild(node);
  };
  option("", t("tray_auto"));
  for (const snap of snapshots) {
    const usable = snap.meters.some((m) => m.primary && m.used_percent !== null);
    if (usable || trayService() === snap.id) option(snap.id, snap.name);
  }
  picker.addEventListener("change", () => {
    setTrayService(picker.value);
    void invoke("set_tray_service", { id: picker.value });
  });
  body.appendChild(
    el("section", "card", "", [el("h2", "", t("tray_number")), picker, el("p", "hint picker-hint", t("tray_hint"))]),
  );

  const updates = el("div", "segmented");
  for (const on of [true, false]) {
    updates.appendChild(
      button(t(on ? "updates_auto" : "updates_off"), autoUpdate() === on ? "seg active" : "seg", () => {
        setAutoUpdate(on);
        void invoke("set_auto_update", { on });
        render();
      }),
    );
  }
  const updatesCard = el("section", "card", "", [
    el("h2", "", t("updates")),
    updates,
    el("p", "hint picker-hint", t("updates_hint")),
  ]);

  const connectable = snapshots.filter((s) => s.connect);
  if (connectable.length) {
    const signin = el("section", "card", "", [el("h2", "", t("signin_services")), el("p", "hint", t("signin_hint"))]);
    for (const snap of connectable) signin.appendChild(connectRow(snap));
    body.appendChild(signin);
  }

  const keys = el("section", "card", "", [el("h2", "", t("api_keys")), el("p", "hint", t("api_keys_hint"))]);
  for (const snap of snapshots.filter((s) => s.key_based)) keys.appendChild(keyRow(snap));
  body.appendChild(keys);

  const auto = snapshots.filter((s) => !s.key_based && !s.connect);
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

  const made = credit();
  body.appendChild(updatesCard);

  const top = el("header", "top", "", [
    button(`‹ ${t("back")}`, "ghost", () => {
      confirmingQuit = false;
      view = "main";
      render();
    }),
    el("h1", "", t("settings")),
  ]);
  const ask = (value: boolean) => () => {
    confirmingQuit = value;
    render();
  };
  const footer = confirmingQuit
    ? el("footer", "footer", "", [
        el("span", "quit-question", t("quit_question")),
        el("span", "footer-actions", "", [
          button(t("cancel"), "ghost", ask(false)),
          button(t("quit"), "danger", () => void invoke("quit")),
        ]),
      ])
    : el("footer", "footer", "", [
        el("span", "stamp", `AImeter ${version}`),
        ...(made ? [made] : []),
        button(t("quit"), "ghost", ask(true)),
      ]);
  return el("div", "screen", "", [top, body, footer]);
}

function render(): void {
  // Do not redraw under the user's fingers while a key is being typed.
  const active = document.activeElement;
  if (active instanceof HTMLInputElement && active.value !== "") return;
  if (active instanceof HTMLSelectElement) active.blur();
  document.documentElement.lang = lang();
  root.replaceChildren(view === "main" ? mainView() : settingsView());
  fit();
}

let fittedHeight = 0;

/** Ask the window to be exactly as tall as the content (within limits). */
function fit(): void {
  const screen = root.firstElementChild;
  if (!screen) return;
  root.classList.add("measuring");
  const natural = Math.ceil(screen.getBoundingClientRect().height);
  root.classList.remove("measuring");
  const height = Math.min(600, Math.max(170, natural));
  if (height !== fittedHeight) {
    fittedHeight = height;
    void invoke("fit_panel", { height });
  }
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

// The question does not linger after the panel was closed.
window.addEventListener("blur", () => {
  if (confirmingQuit) {
    confirmingQuit = false;
    render();
  }
});
void invoke("set_auto_update", { on: autoUpdate() });
void getVersion().then((value) => {
  version = value;
  render();
});
void invoke("set_show_left", { left: showLeft() });
void invoke("set_tray_service", { id: trayService() });
void listen<Snapshot[]>("snapshots", (event) => accept(event.payload));
void invoke<Snapshot[]>("get_snapshots").then(accept);
// Countdown texts tick on their own.
setInterval(() => {
  if (view === "main") render();
}, 30000);
render();
