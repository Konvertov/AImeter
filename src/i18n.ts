export type Lang = "en" | "ru";

const words: Record<Lang, Record<string, string>> = {
  en: {
    session: "Session (5h)",
    weekly: "Weekly",
    monthly: "Monthly",
    limit: "Limit",
    weekly_opus: "Weekly · Opus",
    weekly_sonnet: "Weekly · Sonnet",
    extra_usage: "Extra usage",
    code_review: "Code review",
    credits: "Credits",
    plan_usage: "Plan usage",
    auto_mode: "Auto mode",
    api_models: "Named models",
    on_demand: "On-demand",
    premium: "Premium requests",
    chat: "Chat",
    completions: "Completions",
    key_limit: "Key limit",
    today: "Today",
    this_month: "This month",
    spent: "Spent",
    balance: "Balance",
    used: "used",
    resets_in: "resets in",
    resets_now: "resets any moment",
    updated: "Updated",
    never: "not yet",
    refresh: "Refresh",
    settings: "Settings",
    back: "Back",
    quit: "Quit",
    language: "Language",
    api_keys: "API keys",
    api_keys_hint: "Stored on this computer only.",
    save: "Save",
    remove: "Remove",
    get_key: "Get a key",
    key_saved: "Key saved",
    paste_key: "Paste API key",
    detected: "Found on this computer",
    not_detected: "Not connected",
    not_detected_hint: "Programs appear on their own once installed and signed in. Services marked with a note need a key or a sign-in: click one to set it up.",
    empty_title: "No AI tools found yet",
    empty_hint: "Sign in to an AI tool on this computer (Claude Code, Codex, Cursor and others), or add an API key in Settings.",
    state_expired: "Sign-in expired. Open the program once to renew it.",
    state_expired_connect: "Sign-in expired. Connect again in Settings.",
    state_needs_key: "Add an API key in Settings.",
    state_rate_limited: "The service asked to slow down. Showing the last numbers.",
    state_error: "Could not update",
    loading: "Looking for your AI tools…",
    developed_by: "Developed by",
    hide: "Hide",
    show: "Show",
    hidden_services: "Hidden",
    hidden_hint: "Hidden AI tools are not checked and not shown.",
    show_details: "Details",
    hide_details: "Collapse",
    main_service: "Main",
    make_main: "Make main",
    tab_services: "AI tools",
    tab_look: "Appearance",
    tab_limits: "Limits",
    tab_access: "Sign-in and keys",
    tab_updates: "Updates",
    needs_setup: "Needs setting up",
    set_up: "Set up",
    needs_signin: "sign in once",
    needs_key_short: "needs an API key",
    about: "About",
    theme: "Appearance",
    theme_system: "As system",
    theme_light: "Light",
    theme_dark: "Dark",
    to_light: "Light mode",
    to_dark: "Dark mode",
    updates: "Updates",
    updates_auto: "Install automatically",
    updates_off: "Off",
    updates_hint: "New versions are installed quietly while the panel is closed.",
    quit_question: "Quit AI Meter?",
    cancel: "Cancel",
    tray_number: "Whose limit to show",
    tray_auto: "Automatically",
    tray_hint: "This is the number next to the icon: the session limit. Automatically means the AI tool whose session is closest to its end. Weekly limits are in the panel.",
    signin_services: "Sign in once",
    signin_hint: "A window opens with the service's own site. Sign in there; AI Meter never sees your password.",
    connect: "Connect",
    disconnect: "Disconnect",
    connected: "Connected",
    not_connected: "Not connected",
    left: "left",
    show_as: "Show limits as",
    show_used: "Used",
    show_left: "Left",
    cost_today: "Cost today",
    cost_month: "Cost, last 30 days",
    at_api_prices: "cost is an estimate at API prices",
    cost_partial: "some models have no known price",
    in_reserve: "in reserve",
    over_pace: "over pace",
    on_pace: "On pace",
    runs_out_in: "At this pace runs out in",
    limit_reached: "Limit reached",
    resets_available: "Limit resets available",
    tokens_today: "Tokens today",
    tokens_month: "Tokens, last 30 days",
    top_model: "Top model",
    from_logs: "from local logs",
    daily: "Daily",
    bonus_credits: "Bonus credits",
    quota: "Quota",
    free_tier: "Free tier",
    prompt_credits: "Prompt credits",
    flow_actions: "Flow actions",
    flex_credits: "Flex credits",
    extra_balance: "Extra balance",
    pay_as_you_go: "Pay-as-you-go cap",
    time_format: "Time format",
    clock_12: "12-hour",
    clock_24: "24-hour",
  },
  ru: {
    session: "Сессия (5 ч)",
    weekly: "Неделя",
    monthly: "Месяц",
    limit: "Лимит",
    weekly_opus: "Неделя · Opus",
    weekly_sonnet: "Неделя · Sonnet",
    extra_usage: "Доп. расход",
    code_review: "Код-ревью",
    credits: "Кредиты",
    plan_usage: "Расход по тарифу",
    auto_mode: "Режим Auto",
    api_models: "Выбранные модели",
    on_demand: "Сверх тарифа",
    premium: "Премиум-запросы",
    chat: "Чат",
    completions: "Автодополнения",
    key_limit: "Лимит ключа",
    today: "Сегодня",
    this_month: "В этом месяце",
    spent: "Потрачено",
    balance: "Баланс",
    used: "израсходовано",
    resets_in: "сброс через",
    resets_now: "сброс с минуты на минуту",
    updated: "Обновлено",
    never: "ещё нет",
    refresh: "Обновить",
    settings: "Настройки",
    back: "Назад",
    quit: "Выйти",
    language: "Язык",
    api_keys: "API-ключи",
    api_keys_hint: "Хранятся только на этом компьютере.",
    save: "Сохранить",
    remove: "Удалить",
    get_key: "Получить ключ",
    key_saved: "Ключ сохранён",
    paste_key: "Вставьте API-ключ",
    detected: "Найдено на этом компьютере",
    not_detected: "Не подключено",
    not_detected_hint: "Программы появятся сами после установки и входа. Сервисам с пометкой нужен ключ или вход: нажмите на такой, чтобы настроить.",
    empty_title: "Нейросети пока не найдены",
    empty_hint: "Войдите в любую нейросеть на этом компьютере (Claude Code, Codex, Cursor и другие) либо добавьте API-ключ в настройках.",
    state_expired: "Вход устарел. Откройте программу один раз, чтобы он обновился.",
    state_expired_connect: "Вход устарел. Подключите заново в настройках.",
    state_needs_key: "Добавьте API-ключ в настройках.",
    state_rate_limited: "Сервис попросил подождать. Показаны последние данные.",
    state_error: "Не удалось обновить",
    loading: "Ищем ваши нейросети…",
    developed_by: "Разработано:",
    hide: "Скрыть",
    show: "Показать",
    hidden_services: "Скрытые",
    hidden_hint: "Скрытые нейросети не опрашиваются и не показываются.",
    show_details: "Подробнее",
    hide_details: "Свернуть",
    main_service: "Главная",
    make_main: "Сделать главной",
    tab_services: "Нейросети",
    tab_look: "Оформление",
    tab_limits: "Лимиты",
    tab_access: "Вход и ключи",
    tab_updates: "Обновления",
    needs_setup: "Нужно настроить",
    set_up: "Настроить",
    needs_signin: "войти один раз",
    needs_key_short: "нужен API-ключ",
    about: "О приложении",
    theme: "Оформление",
    theme_system: "Как в системе",
    theme_light: "Светлое",
    theme_dark: "Тёмное",
    to_light: "Светлая тема",
    to_dark: "Тёмная тема",
    updates: "Обновления",
    updates_auto: "Ставить автоматически",
    updates_off: "Выключено",
    updates_hint: "Новые версии ставятся незаметно, пока панель закрыта.",
    quit_question: "Выйти из AI Meter?",
    cancel: "Отмена",
    tray_number: "Лимит какой нейросети показывать",
    tray_auto: "Автоматически",
    tray_hint: "Это число рядом со значком: лимит текущей сессии. «Автоматически» — нейросеть, у которой сессия ближе всего к концу. Недельные лимиты видны в панели.",
    signin_services: "Вход один раз",
    signin_hint: "Откроется окно с сайтом самого сервиса. Войдите там; пароль AI Meter не видит.",
    connect: "Подключить",
    disconnect: "Отключить",
    connected: "Подключено",
    not_connected: "Не подключено",
    left: "осталось",
    show_as: "Показывать лимиты как",
    show_used: "Израсходовано",
    show_left: "Осталось",
    cost_today: "Стоимость сегодня",
    cost_month: "Стоимость за 30 дней",
    at_api_prices: "стоимость — оценка по ценам API",
    cost_partial: "для части моделей цена неизвестна",
    in_reserve: "в запасе",
    over_pace: "сверх темпа",
    on_pace: "В темпе",
    runs_out_in: "При таком темпе закончится через",
    limit_reached: "Лимит исчерпан",
    resets_available: "Доступно сбросов лимита",
    tokens_today: "Токенов сегодня",
    tokens_month: "Токенов за 30 дней",
    top_model: "Чаще всего",
    from_logs: "по журналам на этом компьютере",
    daily: "День",
    bonus_credits: "Бонусные кредиты",
    quota: "Квота",
    free_tier: "Бесплатный лимит",
    prompt_credits: "Кредиты запросов",
    flow_actions: "Действия Flow",
    flex_credits: "Flex-кредиты",
    extra_balance: "Доп. баланс",
    pay_as_you_go: "Потолок оплаты по факту",
    time_format: "Формат времени",
    clock_12: "12 часов",
    clock_24: "24 часа",
  },
};

let current: Lang = "en";
try {
  const saved = localStorage.getItem("lang");
  if (saved === "en" || saved === "ru") current = saved;
} catch {
  /* storage may be unavailable; English stays */
}

export function lang(): Lang {
  return current;
}

export function setLang(next: Lang): void {
  current = next;
  try {
    localStorage.setItem("lang", next);
  } catch {
    /* ignore */
  }
}

/** Translate a key; unknown keys (new limits a service adds) are shown readable as is. */
export function t(key: string): string {
  // Model names and other texts that come from a service are shown as they are.
  return words[current][key] ?? words.en[key] ?? key;
}

export function duration(ms: number): string {
  const minutes = Math.max(0, Math.round(ms / 60000));
  const d = Math.floor(minutes / 1440);
  const h = Math.floor((minutes % 1440) / 60);
  const m = minutes % 60;
  const u = current === "ru" ? { d: "д", h: "ч", m: "мин" } : { d: "d", h: "h", m: "m" };
  if (d > 0) return `${d}${u.d} ${h}${u.h}`;
  if (h > 0) return `${h}${u.h} ${m}${u.m}`;
  return `${m}${u.m}`;
}

export type Clock = "12" | "24";

/** Clock style: the user's choice, otherwise 12-hour in English and 24-hour in Russian. */
export function clock(): Clock {
  try {
    const saved = localStorage.getItem("clock");
    if (saved === "12" || saved === "24") return saved;
  } catch {
    /* ignore */
  }
  return current === "ru" ? "24" : "12";
}

export function setClock(next: Clock): void {
  try {
    localStorage.setItem("clock", next);
  } catch {
    /* ignore */
  }
}

export function timeOfDay(ms: number): string {
  return new Date(ms).toLocaleTimeString(current === "ru" ? "ru-RU" : "en-US", {
    hour: "2-digit",
    minute: "2-digit",
    hour12: clock() === "12",
  });
}

/** Limits shown as spent ("used") or as remaining ("left"). */
export function showLeft(): boolean {
  try {
    return localStorage.getItem("show") === "left";
  } catch {
    return false;
  }
}

export function setShowLeft(left: boolean): void {
  try {
    localStorage.setItem("show", left ? "left" : "used");
  } catch {
    /* ignore */
  }
}

export function money(amount: number): string {
  return new Intl.NumberFormat(current === "ru" ? "ru-RU" : "en-US", {
    style: "currency",
    currency: "USD",
    minimumFractionDigits: 2,
    maximumFractionDigits: 2,
  }).format(amount);
}

/** Service whose limit is shown next to the icon; "" = the tightest of all. */
export function trayService(): string {
  try {
    return localStorage.getItem("tray") ?? "";
  } catch {
    return "";
  }
}

export function setTrayService(id: string): void {
  try {
    localStorage.setItem("tray", id);
  } catch {
    /* ignore */
  }
}

export function autoUpdate(): boolean {
  try {
    return localStorage.getItem("updates") !== "off";
  } catch {
    return true;
  }
}

export function setAutoUpdate(on: boolean): void {
  try {
    localStorage.setItem("updates", on ? "on" : "off");
  } catch {
    /* ignore */
  }
}

export type Theme = "system" | "light" | "dark";

export function theme(): Theme {
  try {
    const saved = localStorage.getItem("theme");
    if (saved === "light" || saved === "dark" || saved === "system") return saved;
  } catch {
    /* ignore */
  }
  // Dark until the user picks otherwise: the panel looks the same everywhere.
  return "dark";
}

/** What is on screen now, light or dark, with "system" resolved. */
export function shownTheme(): "light" | "dark" {
  const chosen = theme();
  if (chosen !== "system") return chosen;
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

/** Remember the choice and repaint: "system" follows the computer's own setting. */
export function setTheme(next: Theme): void {
  try {
    localStorage.setItem("theme", next);
  } catch {
    /* ignore */
  }
  applyTheme();
}

export function applyTheme(): void {
  const chosen = theme();
  if (chosen === "system") delete document.documentElement.dataset.theme;
  else document.documentElement.dataset.theme = chosen;
}

/** The service shown first and fully open; "" = the first one found. */
export function mainService(): string {
  try {
    return localStorage.getItem("main") ?? "";
  } catch {
    return "";
  }
}

export function setMainService(id: string): void {
  try {
    localStorage.setItem("main", id);
  } catch {
    /* ignore */
  }
}

/** Services the user switched off. */
export function hiddenServices(): string[] {
  try {
    const saved = JSON.parse(localStorage.getItem("hidden") ?? "[]");
    return Array.isArray(saved) ? saved.filter((id) => typeof id === "string") : [];
  } catch {
    return [];
  }
}

export function setHiddenServices(ids: string[]): void {
  try {
    localStorage.setItem("hidden", JSON.stringify(ids));
  } catch {
    /* ignore */
  }
}
