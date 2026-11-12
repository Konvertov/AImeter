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
    api_keys_hint: "Stored in your system password store. Never sent anywhere except the service itself.",
    save: "Save",
    remove: "Remove",
    get_key: "Get a key",
    key_saved: "Key saved",
    paste_key: "Paste API key",
    detected: "Found on this computer",
    not_detected: "Not found on this computer",
    not_detected_hint: "Install the program and sign in. It will appear here on its own.",
    empty_title: "No AI tools found yet",
    empty_hint: "Sign in to Claude Code, Codex, Cursor or GitHub CLI, or add an API key in Settings.",
    state_expired: "Sign-in expired. Open the program once to renew it.",
    state_needs_key: "Add an API key in Settings.",
    state_rate_limited: "The service asked to slow down. Showing the last numbers.",
    state_error: "Could not update",
    loading: "Looking for your AI tools…",
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
    api_keys_hint: "Хранятся в системном хранилище паролей. Отправляются только самому сервису.",
    save: "Сохранить",
    remove: "Удалить",
    get_key: "Получить ключ",
    key_saved: "Ключ сохранён",
    paste_key: "Вставьте API-ключ",
    detected: "Найдено на этом компьютере",
    not_detected: "Не найдено на этом компьютере",
    not_detected_hint: "Установите программу и войдите в аккаунт. Она появится здесь сама.",
    empty_title: "Нейросети пока не найдены",
    empty_hint: "Войдите в Claude Code, Codex, Cursor или GitHub CLI либо добавьте API-ключ в настройках.",
    state_expired: "Вход устарел. Откройте программу один раз, чтобы он обновился.",
    state_needs_key: "Добавьте API-ключ в настройках.",
    state_rate_limited: "Сервис попросил подождать. Показаны последние данные.",
    state_error: "Не удалось обновить",
    loading: "Ищем ваши нейросети…",
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
  return words[current][key] ?? words.en[key] ?? key.replace(/_/g, " ");
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
