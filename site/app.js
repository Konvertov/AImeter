// Text in two languages. English is what the page ships with; Russian replaces it on request.
const RU = {
  nav_features: "Возможности",
  nav_services: "Нейросети",
  nav_privacy: "Приватность",
  nav_download: "Скачать",
  eyebrow: "Бесплатно · macOS и Windows",
  hero_title: "Лимиты нейросетей — на виду, пока они не кончились",
  hero_lead: "AI Meter сам находит нейросети, которыми вы уже пользуетесь, и показывает, сколько осталось от каждого лимита и когда он сбросится.",
  dl_mac: "Скачать для Mac",
  dl_win: "Скачать для Windows",
  hero_fine: "Версия 0.2.0 · без аккаунта и регистрации · macOS 11 и новее, Windows 10 и 11",
  m_session: "Сессия (5 ч)",
  m_weekly: "Неделя",
  m_plan: "Расход по тарифу",
  m_reserve: "13% в запасе",
  m_reserve2: "13% в запасе",
  m_over: "28% сверх темпа",
  m_reset1: "сброс через 2ч 14мин",
  m_reset2: "сброс через 4д 18ч",
  m_reset3: "сброс через 2ч 36мин",
  m_reset4: "сброс через 18д",
  m_today: "Токенов сегодня",
  m_month: "Токенов за 30 дней",
  m_details: "Подробнее ⌄",
  f_title: "Что он умеет",
  f1_t: "Сам находит ваши нейросети",
  f1_p: "Если Claude Code, Codex, Cursor или другая поддерживаемая программа установлена и в неё выполнен вход, её карточка просто появляется. Ничего вводить не нужно.",
  f2_t: "Лимиты и сбросы одним взглядом",
  f2_p: "Лимиты на сессию, неделю и месяц в виде полосок с отсчётом до сброса. Самый тесный лимит показан прямо у значка.",
  f3_t: "Предупреждает, если не хватит",
  f3_p: "Риска на полоске показывает, где был бы расход при ровном темпе. Если вы тратите быстрее, AI Meter скажет, когда лимит кончится.",
  f4_t: "Токены по вашим журналам",
  f4_p: "Для Claude Code: токены за сегодня и за 30 дней, столбик на каждый день и модель, которой вы пользуетесь чаще всего. Считается на вашем компьютере.",
  f5_t: "Показывает только нужное",
  f5_p: "Выберите главную нейросеть, сверните остальные до основных лимитов, скройте лишние, смотрите расход или остаток.",
  f6_t: "Тихий и аккуратный",
  f6_p: "Светлое или тёмное оформление, русский или английский. Новые версии ставятся сами, пока панель закрыта.",
  s_title: "Какие нейросети он понимает",
  s_sub: "Большинство находится автоматически. Нескольким нужен API-ключ или один вход.",
  s_fine: "Список растёт с каждой версией.",
  how_auto: "находится сам",
  how_key: "по API-ключу",
  how_signin: "вход один раз",
  h_title: "Как это работает",
  h1_t: "Установите и запустите",
  h1_p: "В строке меню на Mac или в трее Windows появится небольшой значок.",
  h2_t: "Он осматривается",
  h2_p: "AI Meter проверяет, в какие нейросети выполнен вход на этом компьютере, и спрашивает у каждого сервиса ваши лимиты — так же, как это делает сама программа.",
  h3_t: "Вы поглядываете",
  h3_p: "Нажмите на значок, чтобы увидеть все лимиты, или просто смотрите на число рядом с ним.",
  p_title: "Ваши данные остаются у вас",
  p1_t: "Нет нашего сервера",
  p1_p: "AI Meter общается только с самими сервисами нейросетей. Между ними ничего нет, и ничего не собирается.",
  p2_t: "Входы только читаются",
  p2_p: "Приложение читает вход, который ваши программы уже сохранили. Оно его не меняет, никуда не копирует и не выводит вас из аккаунта.",
  p3_t: "Ключи остаются на компьютере",
  p3_p: "API-ключи, которые вы добавили, хранятся локально: в Windows — в системном хранилище паролей, на Mac — в файле, который может читать только ваш пользователь.",
  p4_t: "Никакой статистики",
  p4_p: "Приложение не сообщает, что вы делаете, какими нейросетями пользуетесь и сколько.",
  i_title: "Первый запуск",
  i_mac: "Откройте скачанный файл и перетащите AI Meter в «Программы». В первый раз нажмите на приложение правой кнопкой и выберите «Открыть»: macOS просит подтвердить программы не из App Store.",
  i_win: "Запустите установщик. Если Windows покажет «Система Windows защитила ваш компьютер», нажмите «Подробнее», затем «Выполнить в любом случае». Приложение ставится только для вашего пользователя и не требует прав администратора.",
  title: "AI Meter — лимиты нейросетей рядом с часами",
};

// name, logo file (or "" for a letter), brand colour (or "" to follow the text), how it connects
const SERVICES = [
  ["Claude Code", "claude", "#D97757", "auto"],
  ["Codex", "codex", "", "auto"],
  ["Cursor", "cursor", "", "auto"],
  ["GitHub Copilot", "copilot", "", "auto"],
  ["Gemini", "gemini", "#4E8CF7", "signin"],
  ["Antigravity", "", "", "auto"],
  ["Windsurf", "windsurf", "", "auto"],
  ["Grok", "", "", "auto"],
  ["Kiro", "", "", "auto"],
  ["JetBrains AI", "jetbrains", "", "auto"],
  ["Devin", "", "", "auto"],
  ["Amp", "", "", "auto"],
  ["OpenCode Go", "opencode_go", "", "auto"],
  ["Kimi Code", "kimi", "", "auto"],
  ["Codebuff", "", "", "auto"],
  ["OpenRouter", "openrouter", "#6467F2", "key"],
  ["abliteration.ai", "abliteration", "", "key"],
  ["DeepSeek", "deepseek", "#4D6BFE", "key"],
];
const HOW_EN = { auto: "found automatically", key: "by API key", signin: "one-time sign-in" };

const english = {};
document.querySelectorAll("[data-t]").forEach((node) => {
  english[node.dataset.t] = node.textContent;
});
english.title = document.title;

function drawServices(lang) {
  const list = document.getElementById("service-list");
  list.replaceChildren();
  for (const [name, file, color, how] of SERVICES) {
    const item = document.createElement("div");
    item.className = "service";
    const mark = document.createElement("span");
    if (file) {
      mark.className = "logo";
      mark.style.setProperty("--logo", `url(logos/${file}.svg)`);
      if (color) mark.style.color = color;
    } else {
      mark.className = "letter";
      mark.textContent = name[0];
    }
    const text = document.createElement("span");
    text.textContent = name;
    const note = document.createElement("span");
    note.className = "how";
    note.textContent = lang === "ru" ? RU[`how_${how}`] : HOW_EN[how];
    text.appendChild(note);
    item.append(mark, text);
    list.appendChild(item);
  }
}

function setLanguage(lang) {
  const words = lang === "ru" ? RU : english;
  document.documentElement.lang = lang;
  document.querySelectorAll("[data-t]").forEach((node) => {
    node.textContent = words[node.dataset.t] ?? english[node.dataset.t];
  });
  document.title = words.title;
  document.querySelectorAll("[data-lang]").forEach((button) => {
    button.classList.toggle("active", button.dataset.lang === lang);
  });
  drawServices(lang);
  try {
    localStorage.setItem("lang", lang);
  } catch {
    /* the choice just will not be remembered */
  }
}

document.querySelectorAll("[data-lang]").forEach((button) => {
  button.addEventListener("click", () => setLanguage(button.dataset.lang));
});

// The little chart in the drawn panel.
const chart = document.getElementById("chart");
[18, 9, 24, 13, 16, 62, 6, 0, 3, 100, 2, 27, 0, 4, 11, 22, 8, 0, 14, 31, 5, 0, 19, 44, 12, 7, 0, 26, 9, 48].forEach((height) => {
  const column = document.createElement("span");
  column.style.height = `${Math.max(3, height)}%`;
  chart.appendChild(column);
});

// Put the visitor's own system first among the download buttons.
const onWindows = /Windows/i.test(navigator.userAgent);
const actions = document.getElementById("download");
const mac = actions.querySelector('[data-download="mac"]');
const windows = actions.querySelector('[data-download="windows"]');
if (onWindows) {
  actions.prepend(windows);
  windows.classList.remove("ghost");
  mac.classList.add("ghost");
}

// A download that is not uploaded yet is shown dimmed instead of leading to an error page.
for (const link of [mac, windows]) {
  fetch(link.href, { method: "HEAD" })
    .then((answer) => {
      if (!answer.ok) throw new Error("missing");
    })
    .catch(() => {
      link.classList.add("unavailable");
      link.title = document.documentElement.lang === "ru" ? "Файл скоро появится" : "Coming soon";
      link.addEventListener("click", (event) => event.preventDefault());
    });
}

let saved = "";
try {
  saved = localStorage.getItem("lang") ?? "";
} catch {
  /* ignore */
}
setLanguage(saved || (navigator.language.toLowerCase().startsWith("ru") ? "ru" : "en"));
