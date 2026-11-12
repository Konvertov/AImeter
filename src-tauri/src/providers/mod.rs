//! Every service AImeter can read lives in its own file here.
//! A service module exposes one function: `fetch(client) -> Result<Data, Fail>`.

use serde::Serialize;
use serde_json::Value;

mod claude;
mod codex;
mod copilot;
mod cursor;
mod deepseek;
mod openrouter;

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
}

impl Meter {
    pub fn percent(label: &str, used: f64) -> Self {
        Meter {
            label: label.into(),
            used_percent: Some(used.clamp(0.0, 100.0)),
            detail: None,
            resets_at: None,
            primary: false,
        }
    }
    pub fn text(label: &str, detail: String) -> Self {
        Meter {
            label: label.into(),
            used_percent: None,
            detail: Some(detail),
            resets_at: None,
            primary: false,
        }
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
    pub meters: Vec<Meter>,
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
    pub meters: Vec<Meter>,
    pub fetched_at: i64,
    pub key_based: bool,
    pub has_key: bool,
    pub key_url: Option<String>,
}

pub struct Info {
    pub id: &'static str,
    pub name: &'static str,
    /// Env variable that may hold the API key; None for auto-detected services.
    pub key_env: Option<&'static str>,
    pub key_url: Option<&'static str>,
}

pub const ALL: &[Info] = &[
    Info { id: "claude", name: "Claude", key_env: None, key_url: None },
    Info { id: "codex", name: "Codex", key_env: None, key_url: None },
    Info { id: "cursor", name: "Cursor", key_env: None, key_url: None },
    Info { id: "copilot", name: "GitHub Copilot", key_env: None, key_url: None },
    Info {
        id: "openrouter",
        name: "OpenRouter",
        key_env: Some("OPENROUTER_API_KEY"),
        key_url: Some("https://openrouter.ai/settings/keys"),
    },
    Info {
        id: "deepseek",
        name: "DeepSeek",
        key_env: Some("DEEPSEEK_API_KEY"),
        key_url: Some("https://platform.deepseek.com/api_keys"),
    },
];

pub fn info(id: &str) -> Option<&'static Info> {
    ALL.iter().find(|p| p.id == id)
}

pub async fn snapshot(info: &'static Info, client: &reqwest::Client) -> Snapshot {
    let result = match info.id {
        "claude" => claude::fetch(client).await,
        "codex" => codex::fetch(client).await,
        "cursor" => cursor::fetch(client).await,
        "copilot" => copilot::fetch(client).await,
        "openrouter" => openrouter::fetch(client).await,
        "deepseek" => deepseek::fetch(client).await,
        _ => Err(Fail::NotFound),
    };
    let has_key = info.key_env.is_some() && crate::secrets::api_key(info).is_some();
    let mut snap = Snapshot {
        id: info.id.into(),
        name: info.name.into(),
        state: "ok".into(),
        message: None,
        plan: None,
        meters: vec![],
        fetched_at: now_ms(),
        key_based: info.key_env.is_some(),
        has_key,
        key_url: info.key_url.map(String::from),
    };
    match result {
        Ok(data) => {
            snap.plan = data.plan;
            snap.meters = data.meters;
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
