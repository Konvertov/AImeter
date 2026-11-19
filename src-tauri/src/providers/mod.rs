//! Every service AImeter can read lives in its own file here.
//! A service module exposes one function: `fetch(client) -> Result<Data, Fail>`.

use serde::Serialize;
use serde_json::Value;

mod abliteration;
mod amp;
mod antigravity;
mod claude;
mod claude_logs;
mod prices;
mod codebuff;
mod codex;
mod copilot;
mod cursor;
mod deepseek;
pub mod gemini;
mod devin;
mod elevenlabs;
mod grok;
mod jetbrains;
mod kimi;
mod kiro;
mod opencode_go;
mod openrouter;
mod windsurf;
mod zai;

/// One bar (or one line of text) on a service card.
#[derive(Serialize, Clone, Debug)]
pub struct Meter {
    /// Translation key on the UI side ("session", "weekly", ...).
    pub label: String,
    /// 0..100, how much of the limit is spent. None for text-only lines.
    pub used_percent: Option<f64>,
    /// Free text next to the bar ("$4.20 / $20.00").
    pub detail: Option<String>,
    /// Unix milliseconds of the next reset.
    pub resets_at: Option<i64>,
    /// Shown in the tray number.
    pub primary: bool,
    /// Length of the limit window in seconds, when known (for the pace).
    pub window_secs: Option<i64>,
}

impl Meter {
    pub fn percent(label: &str, used: f64) -> Self {
        Meter {
            label: label.into(),
            used_percent: Some(used.clamp(0.0, 100.0)),
            detail: None,
            resets_at: None,
            primary: false,
            window_secs: None,
        }
    }
    pub fn text(label: &str, detail: String) -> Self {
        Meter {
            label: label.into(),
            used_percent: None,
            detail: Some(detail),
            resets_at: None,
            primary: false,
            window_secs: None,
        }
    }
    /// A bar from "spent X out of Y", with "X / Y" written next to it.
    pub fn of(label: &str, used: f64, limit: f64) -> Self {
        Meter::percent(label, if limit > 0.0 { used / limit * 100.0 } else { 0.0 })
            .detail(format!("{} / {}", short(used), short(limit)))
    }
    pub fn window(mut self, seconds: i64) -> Self {
        if seconds > 0 {
            self.window_secs = Some(seconds);
        }
        self
    }
    pub fn resets(mut self, at: Option<i64>) -> Self {
        self.resets_at = at;
        self
    }
    pub fn detail(mut self, text: String) -> Self {
        self.detail = Some(text);
        self
    }
    pub fn primary(mut self) -> Self {
        self.primary = true;
        self
    }
}

/// What a service returns when everything went well.
#[derive(Default)]
pub struct Data {
    pub plan: Option<String>,
    /// Signed-in account (e-mail), when the saved sign-in tells it.
    pub account: Option<String>,
    pub meters: Vec<Meter>,
    pub billing: Option<Billing>,
}

/// When the paid period ends: the next charge, or the last day of a
/// cancelled subscription.
#[derive(Serialize, Clone, Debug)]
pub struct Billing {
    /// Unix milliseconds.
    pub at: i64,
    /// Worked out from the start date, not reported by the service.
    pub estimate: bool,
    /// The subscription is cancelled and simply runs out at `at`.
    pub ends: bool,
}

impl Billing {
    pub fn renews(at: Option<i64>) -> Option<Self> {
        at.filter(|at| *at > now_ms()).map(|at| Billing { at, estimate: false, ends: false })
    }
}

/// A paid plan: free and trial plans have no charge to remind about.
pub fn paid(plan: &Option<String>) -> bool {
    plan.as_deref().map_or(false, |plan| {
        let plan = plan.to_lowercase();
        !["free", "trial", "hobby"].iter().any(|word| plan.contains(word))
    })
}

/// The first monthly anniversary of `start` that is still ahead.
pub fn next_monthly(start: i64) -> Option<i64> {
    use chrono::{Months, TimeZone};
    let first = chrono::Utc.timestamp_millis_opt(start).single()?;
    let now = now_ms();
    (1..=1200u32)
        .map(|n| first.checked_add_months(Months::new(n)).map(|d| d.timestamp_millis()))
        .find(|at| at.map_or(true, |at| at > now))
        .flatten()
}

/// Why a service has nothing to show.
pub enum Fail {
    /// The program is not installed or nobody signed in: hide the card.
    NotFound,
    /// A saved sign-in exists but the service rejected it.
    Expired,
    /// The service needs an API key and none is saved.
    NeedsKey,
    /// The service asked us to slow down.
    RateLimited,
    /// Anything else, with a short technical note.
    Other(String),
}

#[derive(Serialize, Clone, Debug)]
pub struct Snapshot {
    pub id: String,
    pub name: String,
    /// "ok" | "not_found" | "expired" | "needs_key" | "rate_limited" | "error"
    pub state: String,
    pub message: Option<String>,
    pub plan: Option<String>,
    pub account: Option<String>,
    pub meters: Vec<Meter>,
    pub billing: Option<Billing>,
    /// Token counts from local logs, for services that keep such logs.
    pub stats: Option<claude_logs::Stats>,
    pub fetched_at: i64,
    pub key_based: bool,
    pub has_key: bool,
    pub key_url: Option<String>,
    /// Connected by signing in once in a window (see `has_key` for whether it is).
    pub connect: bool,
}

pub struct Info {
    pub id: &'static str,
    pub name: &'static str,
    /// Env variable that may hold the API key; None for auto-detected services.
    pub key_env: Option<&'static str>,
    pub key_url: Option<&'static str>,
    /// The user connects this service by signing in once in a window.
    pub connect: bool,
}

pub const ALL: &[Info] = &[
    Info { id: "claude", name: "Claude Code", key_env: None, key_url: None, connect: false },
    Info { id: "codex", name: "Codex", key_env: None, key_url: None, connect: false },
    Info { id: "cursor", name: "Cursor", key_env: None, key_url: None, connect: false },
    Info { id: "copilot", name: "GitHub Copilot", key_env: None, key_url: None, connect: false },
    Info { id: "gemini", name: "Gemini", key_env: None, key_url: None, connect: true },
    Info { id: "antigravity", name: "Antigravity", key_env: None, key_url: None, connect: false },
    Info { id: "windsurf", name: "Windsurf", key_env: None, key_url: None, connect: false },
    Info { id: "grok", name: "Grok", key_env: None, key_url: None, connect: false },
    Info { id: "kiro", name: "Kiro", key_env: None, key_url: None, connect: false },
    Info { id: "jetbrains", name: "JetBrains AI", key_env: None, key_url: None, connect: false },
    Info { id: "devin", name: "Devin", key_env: None, key_url: None, connect: false },
    Info { id: "amp", name: "Amp", key_env: None, key_url: None, connect: false },
    Info { id: "opencode_go", name: "OpenCode Go", key_env: None, key_url: None, connect: false },
    Info { id: "kimi", name: "Kimi Code", key_env: None, key_url: None, connect: false },
    Info { id: "codebuff", name: "Codebuff", key_env: None, key_url: None, connect: false },
    Info {
        id: "openrouter",
        name: "OpenRouter",
        key_env: Some("OPENROUTER_API_KEY"),
        key_url: Some("https://openrouter.ai/settings/keys"),
        connect: false,
    },
    Info {
        id: "abliteration",
        name: "abliteration.ai",
        key_env: Some("ABLITERATION_API_KEY"),
        key_url: Some("https://abliteration.ai"),
        connect: false,
    },
    Info {
        id: "deepseek",
        name: "DeepSeek",
        key_env: Some("DEEPSEEK_API_KEY"),
        key_url: Some("https://platform.deepseek.com/api_keys"),
        connect: false,
    },
    Info {
        id: "zai",
        name: "Z.ai",
        key_env: Some("ZAI_API_KEY"),
        key_url: Some("https://z.ai/manage-apikey/apikey-list"),
        connect: false,
    },
    Info {
        id: "elevenlabs",
        name: "ElevenLabs",
        key_env: Some("ELEVENLABS_API_KEY"),
        key_url: Some("https://elevenlabs.io/app/settings/api-keys"),
        connect: false,
    },
];

pub fn info(id: &str) -> Option<&'static Info> {
    ALL.iter().find(|p| p.id == id)
}

/// A card stand-in for a service the user switched off: it is not asked at all.
pub fn hidden(info: &'static Info) -> Snapshot {
    stand_in(info, "hidden", None)
}

/// A card stand-in for a service that did not answer in time.
pub fn timed_out(info: &'static Info) -> Snapshot {
    stand_in(info, "error", Some("timeout".into()))
}

fn stand_in(info: &'static Info, state: &str, message: Option<String>) -> Snapshot {
    Snapshot {
        id: info.id.into(),
        name: info.name.into(),
        state: state.into(),
        message,
        plan: None,
        account: None,
        meters: vec![],
        billing: None,
        stats: None,
        fetched_at: now_ms(),
        key_based: info.key_env.is_some(),
        has_key: false,
        key_url: info.key_url.map(String::from),
        connect: info.connect,
    }
}

pub async fn snapshot(info: &'static Info, client: &reqwest::Client) -> Snapshot {
    let result = match info.id {
        "claude" => claude::fetch(client).await,
        "codex" => codex::fetch(client).await,
        "cursor" => cursor::fetch(client).await,
        "copilot" => copilot::fetch(client).await,
        "gemini" => gemini::fetch(client).await,
        "antigravity" => antigravity::fetch().await,
        "windsurf" => windsurf::fetch().await,
        "grok" => grok::fetch(client).await,
        "kiro" => kiro::fetch().await,
        "jetbrains" => jetbrains::fetch().await,
        "devin" => devin::fetch(client).await,
        "amp" => amp::fetch(client).await,
        "opencode_go" => opencode_go::fetch(client).await,
        "kimi" => kimi::fetch(client).await,
        "codebuff" => codebuff::fetch(client).await,
        "openrouter" => openrouter::fetch(client).await,
        "deepseek" => deepseek::fetch(client).await,
        "zai" => zai::fetch(client).await,
        "elevenlabs" => elevenlabs::fetch(client).await,
        "abliteration" => abliteration::fetch(client).await,
        _ => Err(Fail::NotFound),
    };
    let has_key = if info.connect {
        blocking(move || crate::secrets::stored(info.id)).await.is_some()
    } else {
        info.key_env.is_some() && crate::secrets::api_key(info).is_some()
    };
    let mut snap = Snapshot {
        id: info.id.into(),
        name: info.name.into(),
        state: "ok".into(),
        message: None,
        plan: None,
        account: None,
        meters: vec![],
        billing: None,
        stats: None,
        fetched_at: now_ms(),
        key_based: info.key_env.is_some(),
        has_key,
        key_url: info.key_url.map(String::from),
        connect: info.connect,
    };
    match result {
        Ok(data) => {
            snap.plan = data.plan;
            snap.account = data.account;
            snap.meters = data.meters;
            snap.billing = data.billing;
        }
        Err(Fail::NotFound) => snap.state = "not_found".into(),
        Err(Fail::Expired) => snap.state = "expired".into(),
        Err(Fail::NeedsKey) => snap.state = "needs_key".into(),
        Err(Fail::RateLimited) => snap.state = "rate_limited".into(),
        Err(Fail::Other(note)) => {
            snap.state = "error".into();
            snap.message = Some(note);
        }
    }
    if info.id == "claude" && snap.state != "not_found" {
        snap.stats = blocking(claude_logs::collect).await;
    }
    snap
}

// ---------- helpers shared by the service modules ----------

pub fn now_ms() -> i64 {
    chrono::Utc::now().timestamp_millis()
}

/// Run file / keychain reads off the async threads.
pub async fn blocking<T, F>(work: F) -> Option<T>
where
    F: FnOnce() -> Option<T> + Send + 'static,
    T: Send + 'static,
{
    tauri::async_runtime::spawn_blocking(work).await.ok().flatten()
}

/// Send a request and turn the usual failure codes into `Fail`.
pub async fn send_json(request: reqwest::RequestBuilder) -> Result<Value, Fail> {
    let response = request
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| Fail::Other(if e.is_timeout() { "timeout".into() } else { "network".into() }))?;
    let code = response.status().as_u16();
    match code {
        200..=299 => response
            .json::<Value>()
            .await
            .map_err(|_| Fail::Other("unexpected response".into())),
        401 | 403 => Err(Fail::Expired),
        429 => Err(Fail::RateLimited),
        _ => Err(Fail::Other(format!("HTTP {code}"))),
    }
}

/// "2026-01-28T15:00:00Z" or "2026-01-28" to unix milliseconds.
pub fn iso_ms(text: &str) -> Option<i64> {
    if let Ok(moment) = chrono::DateTime::parse_from_rfc3339(text) {
        return Some(moment.timestamp_millis());
    }
    chrono::NaiveDate::parse_from_str(text, "%Y-%m-%d")
        .ok()
        .and_then(|day| day.and_hms_opt(0, 0, 0))
        .map(|naive| naive.and_utc().timestamp_millis())
}

/// Numbers sometimes arrive as strings.
pub fn number(value: &Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_str().and_then(|s| s.trim().parse().ok()))
        .filter(|n| n.is_finite())
}

pub fn dollars(amount: f64) -> String {
    format!("${amount:.2}")
}

pub fn title_case(text: &str) -> String {
    let mut chars = text.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// 1234.5 -> "1234.5", 20000000 -> "20M": compact amounts for "X / Y".
pub fn short(amount: f64) -> String {
    let abs = amount.abs();
    if abs >= 1_000_000.0 {
        format!("{:.1}M", amount / 1_000_000.0).replace(".0M", "M")
    } else if abs >= 10_000.0 {
        format!("{:.0}K", amount / 1000.0)
    } else if amount.fract().abs() < 0.005 {
        format!("{amount:.0}")
    } else {
        format!("{amount:.1}")
    }
}

/// A moment given as text or number, in seconds or milliseconds, to unix ms.
pub fn moment_ms(value: &Value) -> Option<i64> {
    if let Some(text) = value.as_str() {
        if let Some(ms) = iso_ms(text) {
            return Some(ms);
        }
    }
    let n = number(value)?;
    if n <= 0.0 {
        return None;
    }
    Some(if n > 100_000_000_000.0 { n as i64 } else { (n * 1000.0) as i64 })
}

pub fn home() -> Option<std::path::PathBuf> {
    dirs::home_dir()
}

/// First value stored under `key` anywhere inside a JSON tree.
pub fn deep<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    match value {
        Value::Object(fields) => fields
            .get(key)
            .or_else(|| fields.values().find_map(|inner| deep(inner, key))),
        Value::Array(items) => items.iter().find_map(|inner| deep(inner, key)),
        _ => None,
    }
}

/// Read one row of text from an SQLite file without changing it.
pub fn sqlite_text(file: &std::path::Path, sql: &str, param: &str) -> Option<String> {
    if !file.exists() {
        return None;
    }
    let db = rusqlite::Connection::open_with_flags(
        file,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .ok()?;
    let _ = db.busy_timeout(std::time::Duration::from_secs(2));
    db.query_row(sql, [param], |row| row.get::<_, String>(0))
        .ok()
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
}

/// Editors built on VS Code keep their state in the same kind of file:
/// `<settings folder>/<App>/User/globalStorage/state.vscdb`.
pub fn editor_state(app: &str, key: &str) -> Option<String> {
    let file = dirs::config_dir()?
        .join(app)
        .join("User")
        .join("globalStorage")
        .join("state.vscdb");
    sqlite_text(&file, "SELECT value FROM ItemTable WHERE key = ?1", key)
}

/// Run a helper program without flashing a console window on Windows.
pub fn quiet_command(program: &str) -> std::process::Command {
    #[allow(unused_mut)]
    let mut command = std::process::Command::new(program);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    command
}

pub const HOUR: i64 = 3600;
pub const DAY: i64 = 86400;

/// The "email" written inside a sign-in token (the middle part is plain JSON).
pub fn token_email(token: &str) -> Option<String> {
    use base64::Engine;
    let middle = token.split('.').nth(1)?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(middle.trim_end_matches('='))
        .ok()?;
    let claims: Value = serde_json::from_slice(&bytes).ok()?;
    claims["email"].as_str().map(String::from)
}

/// Everything written inside a sign-in token, as JSON.
pub fn token_claims(token: &str) -> Option<Value> {
    use base64::Engine;
    let middle = token.split('.').nth(1)?;
    let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(middle.trim_end_matches('='))
        .ok()?;
    serde_json::from_slice(&bytes).ok()
}
