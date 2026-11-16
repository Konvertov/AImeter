import { getVersion } from "@tauri-apps/api/app";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { author } from "./author";
import { applyTheme, autoUpdate, clock, hiddenServices, setHiddenServices, mainService, setMainService, duration, lang, setAutoUpdate, setClock, setLang, setShowLeft, setTheme, setTrayService, showLeft, shownTheme, t, theme, timeOfDay, trayService } from "./i18n";
import { logo } from "./logos";
import kvadrik from "./logos/kvadrik.svg?raw";

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
  state: "ok" | "not_found" | "expired" | "needs_key" | "rate_limited" | "error" | "hidden";
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
  services:
    '<rect x="4" y="4" width="6.5" height="6.5" rx="1.5"/><rect x="13.5" y="4" width="6.5" height="6.5" rx="1.5"/>' +
    '<rect x="4" y="13.5" width="6.5" height="6.5" rx="1.5"/><rect x="13.5" y="13.5" width="6.5" height="6.5" rx="1.5"/>',
  look: '<circle cx="12" cy="12" r="8.5"/><path d="M12 3.5v17a8.5 8.5 0 0 0 0-17z" fill="currentColor"/>',
  limits: '<path d="M4 17a8 8 0 1 1 16 0"/><path d="M12 17l4.2-5.4"/><circle cx="12" cy="17" r="1.2" fill="currentColor"/>',
  access: '<circle cx="8" cy="15.5" r="4"/><path d="M10.9 12.6L19.5 4M15.5 8l3 3"/>',
  updates: '<path d="M12 4v11M7.5 10.5L12 15l4.5-4.5M5 20h14"/>',
  refresh: '<path d="M20 12a8 8 0 1 1-2.6-5.9"/><path d="M20 4v5h-5"/>',
  // A solid sun with short rays apart from it: unlike the gear's outline.
  sun:
    '<circle cx="12" cy="12" r="3.6" fill="currentColor"/>' +
    '<path d="M12 2.5v1.8M12 19.7v1.8M2.5 12h1.8M19.7 12h1.8M5.3 5.3l1.3 1.3M17.4 17.4l1.3 1.3M5.3 18.7l1.3-1.3M17.4 6.6l1.3-1.3"/>',
  moon: '<path d="M20 14.5A8 8 0 1 1 9.5 4a6.5 6.5 0 0 0 10.5 10.5z"/>',
  // A gear with real teeth, so it never reads as the sun.
  settings: '<path d="M19.2 12.9 L21.5 14.4 L20.4 17.1 L17.8 16.5 L16.5 17.8 L17.1 20.4 L14.4 21.5 L12.9 19.2 L11.1 19.2 L9.6 21.5 L6.9 20.4 L7.5 17.8 L6.2 16.5 L3.6 17.1 L2.5 14.4 L4.8 12.9 L4.8 11.1 L2.5 9.6 L3.6 6.9 L6.2 7.5 L7.5 6.2 L6.9 3.6 L9.6 2.5 L11.1 4.8 L12.9 4.8 L14.4 2.5 L17.1 3.6 L16.5 6.2 L17.8 7.5 L20.4 6.9 L21.5 9.6 L19.2 11.1Z"/><circle cx="12" cy="12" r="3"/>',
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

/** The app's character, Kvadrik, in front of the name. */
function brandMark(): HTMLElement {
  const mark = el("span", "brand-mark");
  mark.setAttribute("aria-hidden", "true");
  mark.innerHTML = kvadrik;
  return mark;
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

/** Cards the user opened or closed by hand: card key -> open? */
const opened = new Map<string, boolean>();

/**
 * A service card. The main service shows everything; the others show only
 * their main limits, with the rest (extra limits, token counts) folded away
 * behind a "Details" line.
 */
function card(snap: Snapshot, key: string, isMain: boolean): HTMLElement {
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
  // Always visible: the main limits (or the first line when there are none).
  const primary = snap.meters.filter((m) => m.primary);
  const always = primary.length ? primary : snap.meters.slice(0, 1);
  const extra = snap.meters.filter((m) => !always.includes(m));
  for (const meter of always) node.appendChild(meterRow(meter));

  if (extra.length || snap.stats) {
    const open = opened.get(key) ?? isMain;
    if (open) {
      for (const meter of extra) node.appendChild(meterRow(meter));
      if (snap.stats) node.appendChild(statsBlock(snap.stats));
    }
    const toggle = button(`${t(open ? "hide_details" : "show_details")} ${open ? "⌃" : "⌄"}`, "fold", () => {
      opened.set(key, !open);
      render();
    });
    node.appendChild(toggle);
  }
  return node;
}

/** Services in display order: the chosen main one first. */
function ordered(list: Snapshot[]): Snapshot[] {
  const main = mainService();
  const at = list.findIndex((s) => s.id === main);
  if (at <= 0) return list;
  return [list[at], ...list.slice(0, at), ...list.slice(at + 1)];
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
  const off = hiddenServices();
  const shown = ordered(
    snapshots.filter((s) => !["not_found", "needs_key", "hidden"].includes(s.state) && !off.includes(s.id)),
  );
  const list = el("div", "list");
  if (!loaded) {
    list.appendChild(el("div", "empty", "", [el("p", "empty-title", t("loading"))]));
  } else if (shown.length === 0) {
    list.appendChild(
      el("div", "empty", "", [el("p", "empty-title", t("empty_title")), el("p", "empty-hint", t("empty_hint"))]),
    );
  } else {
    shown.forEach((snap, index) => list.appendChild(card(snap, `${snap.id}#${index}`, index === 0)));
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
  // Sun or moon in the corner: one click between light and dark.
  const dark = shownTheme() === "dark";
  const flip = iconButton(dark ? "sun" : "moon", t(dark ? "to_light" : "to_dark"), () => {
    setTheme(dark ? "light" : "dark");
    render();
  });
  flip.classList.add("theme-flip");
  const top = el("header", "top", "", [
    brandMark(),
    el("h1", "", "", [el("span", "brand-ai", "AI"), document.createTextNode(" Meter")]),
    flip,
  ]);
  return el("div", "screen", "", [top, list, footer]);
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

type Tab = "services" | "look" | "limits" | "access" | "updates";
let tab: Tab = "services";

/** One titled block of a settings page. */
function block(title: string, children: Node[]): HTMLElement {
  return el("section", "card", "", [el("h2", "", t(title)), ...children]);
}

/** A row of choices where exactly one is on. */
function choice<T>(options: readonly T[], current: T, label: (value: T) => string, pick: (value: T) => void): HTMLElement {
  const row = el("div", "segmented");
  for (const value of options) {
    row.appendChild(
      button(label(value), value === current ? "seg active" : "seg", () => {
        pick(value);
        render();
      }),
    );
  }
  return row;
}

/** A service line for the lists on the first settings page. */
function serviceLine(snap: Snapshot, note = "", action?: HTMLElement): HTMLElement {
  const name = el("span", "service-name", "", [el("span", "", snap.name)]);
  if (note) name.appendChild(el("span", "account", note));
  const line = el("div", "service-line", "", [logo(snap.id, snap.name), name]);
  if (action) line.appendChild(action);
  return line;
}

/** Switch a service off (hide) or back on, and tell the background part. */
function setHidden(id: string, hide: boolean): void {
  const ids = hiddenServices().filter((other) => other !== id);
  if (hide) ids.push(id);
  setHiddenServices(ids);
  void invoke("set_hidden_services", { ids });
  render();
}

/** First page: what was found on this computer, what needs setting up, what was not found. */
function servicesPage(): HTMLElement[] {
  const off = hiddenServices();
  const isOff = (s: Snapshot) => s.state === "hidden" || off.includes(s.id);
  const needsSetup = (s: Snapshot) => !isOff(s) && (s.key_based || s.connect) && !s.has_key;
  const found = snapshots.filter((s) => !isOff(s) && s.state !== "not_found" && !needsSetup(s));
  const setup = snapshots.filter(needsSetup);
  const missing = snapshots.filter((s) => !isOff(s) && s.state === "not_found" && !needsSetup(s));
  const hidden = snapshots.filter(isOff);
  const pages: HTMLElement[] = [];

  const foundBlock = block("detected", []);
  if (found.length) {
    for (const snap of found) {
      const note = [snap.plan, snap.account].filter(Boolean).join(" · ");
      const isMain = ordered(found)[0] === snap;
      const mark = isMain
        ? el("span", "main-mark", t("main_service"))
        : button(t("make_main"), "link", () => {
            setMainService(snap.id);
            opened.clear();
            render();
          });
      const actions = el("span", "line-actions", "", [
        ...(found.length > 1 ? [mark] : []),
        button(t("hide"), "link quiet", () => setHidden(snap.id, true)),
      ]);
      foundBlock.appendChild(serviceLine(snap, note, actions));
    }
  } else {
    foundBlock.appendChild(el("p", "hint", t(loaded ? "empty_title" : "loading")));
  }
  pages.push(foundBlock);

  if (hidden.length) {
    const hiddenBlock = block("hidden_services", []);
    for (const snap of hidden) {
      hiddenBlock.appendChild(serviceLine(snap, "", button(t("show"), "link", () => setHidden(snap.id, false))));
    }
    hiddenBlock.appendChild(el("p", "hint picker-hint", t("hidden_hint")));
    pages.push(hiddenBlock);
  }

  // Everything not connected yet, in one place: tools that appear on their
  // own once installed, and services that need a key or a sign-in.
  const notConnected = snapshots.filter((s) => setup.includes(s) || missing.includes(s));
  if (notConnected.length) {
    const missingBlock = block("not_detected", []);
    const grid = el("div", "service-grid");
    for (const snap of notConnected) {
      if (setup.includes(snap)) {
        const line = serviceLine(snap, t(snap.connect ? "needs_signin" : "needs_key_short"));
        line.classList.add("clickable");
        line.title = t("set_up");
        line.addEventListener("click", () => {
          tab = "access";
          render();
        });
        grid.appendChild(line);
      } else {
        grid.appendChild(serviceLine(snap));
      }
    }
    missingBlock.append(grid, el("p", "hint picker-hint", t("not_detected_hint")));
    pages.push(missingBlock);
  }
  return pages;
}

function lookPage(): HTMLElement[] {
  return [
    block("language", [
      choice(["en", "ru"] as const, lang(), (code) => (code === "en" ? "English" : "Русский"), setLang),
    ]),
    block("theme", [choice(["system", "light", "dark"] as const, theme(), (value) => t(`theme_${value}`), setTheme)]),
    block("time_format", [choice(["12", "24"] as const, clock(), (value) => t(`clock_${value}`), setClock)]),
  ];
}

function limitsPage(): HTMLElement[] {
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
  return [
    block("show_as", [
      choice([false, true] as const, showLeft(), (left) => t(left ? "show_left" : "show_used"), (left) => {
        setShowLeft(left);
        void invoke("set_show_left", { left });
      }),
    ]),
    block("tray_number", [picker, el("p", "hint picker-hint", t("tray_hint"))]),
  ];
}

function accessPage(): HTMLElement[] {
  const pages: HTMLElement[] = [];
  const connectable = snapshots.filter((s) => s.connect);
  if (connectable.length) {
    pages.push(block("signin_services", [el("p", "hint", t("signin_hint")), ...connectable.map(connectRow)]));
  }
  pages.push(
    block("api_keys", [el("p", "hint", t("api_keys_hint")), ...snapshots.filter((s) => s.key_based).map(keyRow)]),
  );
  return pages;
}

function updatesPage(): HTMLElement[] {
  return [
    block("updates", [
      choice([true, false] as const, autoUpdate(), (on) => t(on ? "updates_auto" : "updates_off"), (on) => {
        setAutoUpdate(on);
        void invoke("set_auto_update", { on });
      }),
      el("p", "hint picker-hint", t("updates_hint")),
    ]),
    // The legal notices the GPL asks an interactive program to show, with the author link.
    block("about", [
      el("p", "names", `AI Meter ${version}`),
      el("p", "hint", t("legal")),
      ...(credit() ? [credit() as HTMLElement] : []),
    ]),
  ];
}

const TABS: { id: Tab; title: string; page: () => HTMLElement[] }[] = [
  { id: "services", title: "tab_services", page: servicesPage },
  { id: "look", title: "tab_look", page: lookPage },
  { id: "limits", title: "tab_limits", page: limitsPage },
  { id: "access", title: "tab_access", page: accessPage },
  { id: "updates", title: "tab_updates", page: updatesPage },
];

function settingsView(): HTMLElement {
  const current = TABS.find((entry) => entry.id === tab) ?? TABS[0];

  // Icons down the left side switch between the pages.
  const rail = el("nav", "rail");
  for (const entry of TABS) {
    const item = iconButton(entry.id, t(entry.title), () => {
      tab = entry.id;
      confirmingQuit = false;
      render();
    });
    if (entry.id === current.id) item.classList.add("active");
    rail.appendChild(item);
  }

  const page = el("div", "list page", "", current.page());
  const made = credit();
  const top = el("header", "top", "", [
    button(`‹ ${t("back")}`, "ghost", () => {
      confirmingQuit = false;
      view = "main";
      render();
    }),
    el("h1", "", t(current.title)),
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
        el("span", "stamp", `AI Meter ${version}`),
        ...(made ? [made] : []),
        button(t("quit"), "ghost", ask(true)),
      ]);
  return el("div", "screen", "", [top, el("div", "settings", "", [rail, page]), footer]);
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

/** On Windows the page is drawn smaller (see PANEL_ZOOM in lib.rs), so the
 *  window needs that share of the page's height. */
const ZOOM = navigator.userAgent.includes("Windows") ? 0.9 : 1;
/** The tallest the panel gets, in page pixels; longer pages scroll. */
const MAX_HEIGHT = 600;

/** The page's natural height, as if the window had no limit. */
function measure(): number {
  const screen = root.firstElementChild;
  if (!screen) return 0;
  root.classList.add("measuring");
  const natural = Math.ceil(screen.getBoundingClientRect().height);
  root.classList.remove("measuring");
  return natural;
}

/** The main view's height the last time it was shown. */
let mainHeight = 0;
/** Settings never get shorter than this, so their pages have room. */
const SETTINGS_MIN = 430;

/** Ask the window to be exactly as tall as the content (within limits).
 *  Settings keep one height for all their tabs (the main view's, but not
 *  less than SETTINGS_MIN), so neither opening them nor switching tabs makes
 *  the window jump; a longer tab scrolls inside. */
function fit(): void {
  let natural: number;
  if (view === "settings") {
    natural = Math.max(mainHeight, SETTINGS_MIN);
  } else {
    natural = measure();
    mainHeight = natural;
  }
  const height = Math.round(Math.min(MAX_HEIGHT, Math.max(170, natural + 2)) * ZOOM);
  if (height !== fittedHeight) {
    fittedHeight = height;
    void invoke("fit_panel", { height });
  } else {
    requestAnimationFrame(topUp);
  }
}

/** After the window took its new size, the engine can still lay the page
 *  out a few pixels taller than measured; grow by what is missing instead
 *  of showing a scrollbar. */
function topUp(): void {
  const list = root.querySelector<HTMLElement>(".list");
  const ceiling = Math.round(MAX_HEIGHT * ZOOM);
  // Settings keep their fixed height and scroll instead.
  if (!list || view === "settings" || fittedHeight >= ceiling) return;
  const missing = list.scrollHeight - list.clientHeight;
  if (missing > 0 && missing < 80) {
    fittedHeight = Math.min(ceiling, fittedHeight + Math.ceil(missing * ZOOM) + 1);
    void invoke("fit_panel", { height: fittedHeight });
  }
}
window.addEventListener("resize", () => requestAnimationFrame(topUp));

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
applyTheme();
void invoke("set_auto_update", { on: autoUpdate() });
void getVersion().then((value) => {
  version = value;
  render();
});
void invoke("set_hidden_services", { ids: hiddenServices() });
void invoke("set_show_left", { left: showLeft() });
void invoke("set_tray_service", { id: trayService() });
void listen<Snapshot[]>("snapshots", (event) => accept(event.payload));
void invoke<Snapshot[]>("get_snapshots").then(accept);
// Countdown texts tick on their own.
setInterval(() => {
  if (view === "main") render();
}, 30000);
render();
