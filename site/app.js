// Text in two languages. English is what the page ships with; Russian replaces it on request.
const RU = {
  nav_features: "Возможности",
  nav_services: "Нейросети",
  nav_privacy: "Приватность",
  nav_download: "Скачать",
  eyebrow: "Бесплатно · macOS и Windows",
  hero_title: "Все лимиты AI‑сервисов —\nв одном месте",
  hero_sub: "Контролируйте лимиты нейросетей, пока они не закончились",
  hero_lead: "AI Meter автоматически находит поддерживаемые AI-инструменты, которыми вы пользуетесь, и собирает информацию об их лимитах в одном удобном интерфейсе.\n\nСледите за оставшимся количеством запросов, использованием токенов и временем до следующего сброса. Больше не нужно открывать каждую нейросеть отдельно, чтобы проверить доступные лимиты.",
  dl_mac: "Скачать для macOS",
  dl_win: "Скачать для Windows",
  hero_fine: "Версия 0.3.4 · Бесплатно · Без регистрации и аккаунта · macOS 11 и новее · Windows 10 и 11",
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
  f_title: "Возможности AI Meter",
  f1_t: "Автоматическое обнаружение AI-инструментов",
  f1_p: "AI Meter самостоятельно определяет, какие поддерживаемые AI-приложения установлены на вашем компьютере и в каких из них выполнен вход. Найденные сервисы автоматически появляются в приложении. Не нужно вручную добавлять их или настраивать подключение.",
  f2_t: "Все лимиты и время сброса на одном экране",
  f2_p: "Отслеживайте ограничения на сессии, неделю и месяц. Наглядные индикаторы показывают, сколько ресурсов уже использовано и сколько осталось. Рядом отображается время до обновления каждого лимита.\n\nСамый актуальный показатель доступен прямо по значку приложения.",
  f3_t: "Прогноз расхода и предупреждения",
  f3_p: "AI Meter анализирует текущую скорость использования лимитов и показывает, насколько её хватит до следующего сброса.\n\nСпециальный индикатор помогает оценить предполагаемый расход. Если вы используете ресурсы быстрее обычного, приложение предупредит о возможном исчерпании лимита.",
  f4_t: "Статистика использования токенов",
  f4_p: "Для Claude Code доступна подробная статистика использования: количество токенов за текущий день и последние 30 дней, ежедневная динамика и наиболее часто используемая модель.\n\nВсе данные рассчитываются локально на вашем компьютере на основе журналов использования.",
  f5_t: "Настройка под себя",
  f5_p: "Выберите основной AI-сервис, чтобы всегда видеть его показатели в первую очередь. Остальные инструменты можно свернуть, скрыть или оставить только основные показатели.\n\nТакже доступны переключение между отображением израсходованных и оставшихся ресурсов и другие настройки интерфейса.",
  f6_t: "Удобный интерфейс и автоматические обновления",
  f6_p: "Выбирайте светлую или тёмную тему, переключайтесь между русским и английским языками.\n\nAI Meter работает в фоновом режиме: на macOS — в строке меню, на Windows — в системном трее. Обновления устанавливаются автоматически, пока панель приложения закрыта.",
  s_title: "Поддерживаемые AI-сервисы",
  s_sub: "AI Meter постоянно расширяет список поддерживаемых инструментов. Большинство из них обнаруживается автоматически — без ручной настройки и дополнительных действий.",
  s_fine: "Список поддерживаемых сервисов регулярно пополняется.",
  group_auto: "Автоматическое обнаружение:",
  group_signin: "Требуется авторизация:",
  group_key: "Подключение по API-ключу:",
  how_signin: "однократный вход в аккаунт",
  h_title: "Как работает AI Meter",
  h_sub: "Всего три шага — и все ваши AI-лимиты под контролем.",
  h1_t: "Установите приложение",
  h1_p: "Скачайте AI Meter для своей операционной системы и запустите его. Значок приложения появится в строке меню macOS или системном трее Windows.",
  h2_t: "Дождитесь обнаружения сервисов",
  h2_p: "AI Meter проверит наличие поддерживаемых AI-инструментов и доступные данные авторизации. После обнаружения сервисы появятся в общем списке. Для некоторых из них может потребоваться дополнительное подключение.",
  h3_t: "Контролируйте использование",
  h3_p: "Откройте панель приложения, чтобы посмотреть актуальные лимиты, статистику использования и время до следующего сброса. Основные показатели доступны прямо по значку приложения.",
  p_title: "Ваши данные остаются на вашем компьютере",
  p_sub: "AI Meter создан с учётом конфиденциальности. Приложению не нужен собственный сервер, а ваши данные об использовании AI-сервисов не отправляются разработчикам.",
  p1_t: "Без собственного сервера",
  p1_p: "AI Meter взаимодействует непосредственно с поддерживаемыми AI-сервисами. Промежуточный сервер разработчика не используется, пользовательская статистика не собирается.",
  p2_t: "Доступ только для чтения",
  p2_p: "Приложение использует уже сохранённые данные авторизации поддерживаемых программ. Оно не изменяет их, не копирует в сторонние сервисы и не выполняет выход из ваших аккаунтов.",
  p3_t: "API-ключи хранятся локально",
  p3_p: "Добавленные API-ключи остаются на вашем устройстве. В Windows они хранятся в системном хранилище учётных данных, а в macOS — в локальном файле с ограниченным доступом для вашего пользователя.",
  p4_t: "Никакого сбора статистики",
  p4_p: "AI Meter не отслеживает вашу работу, не собирает историю использования нейросетей и не передаёт разработчикам информацию о ваших запросах и расходе ресурсов.",
  i_title: "Установка и первый запуск",
  i_mac: "1. Скачайте установочный файл AI Meter для macOS.\n2. Откройте DMG-файл и перетащите приложение в папку «Программы».\n3. При первом запуске нажмите на приложение правой кнопкой мыши и выберите «Открыть».\n4. Подтвердите запуск, если macOS покажет предупреждение системы безопасности.\n\nЭто стандартная процедура для приложений, распространяемых за пределами App Store.",
  i_win: "1. Скачайте установщик AI Meter для Windows.\n2. Запустите EXE-файл и следуйте инструкциям установщика.\n3. Если появится окно «Система Windows защитила ваш компьютер», нажмите «Подробнее», а затем «Выполнить в любом случае».\n\nПриложение устанавливается только для текущего пользователя и не требует прав администратора.",
  cta_title: "Все ваши AI-инструменты. Все лимиты. Одно приложение.",
  cta_p: "Больше не нужно переключаться между десятками сервисов и проверять ограничения вручную. Установите AI Meter и держите использование своих AI-инструментов под контролем.",
  cta_note: "Бесплатно. Без регистрации. Для macOS и Windows.",
  title: "AI Meter — лимиты нейросетей рядом с часами",
  to_light: "Светлая тема",
  to_dark: "Тёмная тема",
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

const GROUPS_EN = { auto: "Found automatically", signin: "One-time sign-in", key: "By API key" };
const NAMES_RU = { Codex: "OpenAI Codex", Antigravity: "Google Antigravity" };

function drawServices(lang) {
  const list = document.getElementById("service-list");
  list.replaceChildren();
  for (const group of ["auto", "signin", "key"]) {
    const title = document.createElement("h3");
    title.className = "service-group";
    title.textContent = lang === "ru" ? RU[`group_${group}`] : GROUPS_EN[group];
    const grid = document.createElement("div");
    grid.className = "services";
    for (const [name, file, color, how] of SERVICES) {
      if (how !== group) continue;
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
      text.textContent = lang === "ru" ? NAMES_RU[name] ?? name : name;
      if (how === "signin") {
        const note = document.createElement("span");
        note.className = "how";
        note.textContent = lang === "ru" ? RU.how_signin : HOW_EN.signin;
        text.appendChild(note);
      }
      item.append(mark, text);
      grid.appendChild(item);
    }
    list.append(title, grid);
  }
}

// Light or dark: dark unless the visitor chose light (remembered).
const SUN =
  '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" aria-hidden="true">' +
  '<circle cx="12" cy="12" r="3.6" fill="currentColor"/>' +
  '<path d="M12 2.5v1.8M12 19.7v1.8M2.5 12h1.8M19.7 12h1.8M5.3 5.3l1.3 1.3M17.4 17.4l1.3 1.3M5.3 18.7l1.3-1.3M17.4 6.6l1.3-1.3"/></svg>';
const MOON =
  '<svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linejoin="round" aria-hidden="true">' +
  '<path d="M20 14.5A8 8 0 1 1 9.5 4a6.5 6.5 0 0 0 10.5 10.5z"/></svg>';
const themeButton = document.getElementById("theme-flip");

function drawThemeButton() {
  const light = document.documentElement.dataset.theme === "light";
  const ru = document.documentElement.lang === "ru";
  themeButton.innerHTML = light ? MOON : SUN;
  const label = light ? (ru ? RU.to_dark : "Dark mode") : ru ? RU.to_light : "Light mode";
  themeButton.title = label;
  themeButton.setAttribute("aria-label", label);
}

themeButton.addEventListener("click", () => {
  const light = document.documentElement.dataset.theme !== "light";
  if (light) document.documentElement.dataset.theme = "light";
  else delete document.documentElement.dataset.theme;
  try {
    localStorage.setItem("theme", light ? "light" : "dark");
  } catch {
    /* the choice just will not be remembered */
  }
  drawThemeButton();
});

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
  drawThemeButton();
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
