//! Codex / ChatGPT: the sign-in the Codex CLI keeps on this computer.

use super::*;
use crate::secrets;

const USAGE_URL: &str = "https://chatgpt.com/backend-api/wham/usage";

fn saved_login() -> Option<String> {
    let mut places = vec![];
    if let Some(custom) = std::env::var_os("CODEX_HOME") {
        places.push(std::path::PathBuf::from(custom));
    }
    if let Some(home) = dirs::home_dir() {
        places.push(home.join(".config").join("codex"));
        places.push(home.join(".codex"));
    }
    for folder in places {
        if let Ok(text) = std::fs::read_to_string(folder.join("auth.json")) {
            return Some(text);
        }
    }
    secrets::system_secret("Codex Auth")
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let raw = blocking(saved_login).await.ok_or(Fail::NotFound)?;
    let login: Value = serde_json::from_str(&raw).map_err(|_| Fail::NotFound)?;
    let tokens = &login["tokens"];
    let token = tokens["access_token"].as_str().ok_or(Fail::NotFound)?;

    let mut request = client.get(USAGE_URL).bearer_auth(token);
    if let Some(account) = tokens["account_id"].as_str() {
        request = request.header("ChatGPT-Account-Id", account);
    }
    let body = send_json(request).await?;

    let mut data = Data {
        plan: body["plan_type"].as_str().map(title_case),
        meters: vec![],
    };
    for slot in ["primary_window", "secondary_window"] {
        if let Some(meter) = window(&body["rate_limit"][slot], None) {
            data.meters.push(meter.primary());
        }
    }
    if let Some(meter) = window(&body["code_review_rate_limit"]["primary_window"], Some("code_review")) {
        data.meters.push(meter);
    }
    let credits = &body["credits"];
    if credits["unlimited"].as_bool() == Some(true) {
        data.meters.push(Meter::text("credits", "∞".into()));
    } else if credits["has_credits"].as_bool() == Some(true) {
        if let Some(balance) = number(&credits["balance"]) {
            data.meters.push(Meter::text("credits", format!("{}", balance.floor())));
        }
    }
    Ok(data)
}

/// The service says how long each window is; name it from that.
fn window(value: &Value, fixed_label: Option<&str>) -> Option<Meter> {
    let used = number(&value["used_percent"])?;
    let label = fixed_label.unwrap_or(match number(&value["limit_window_seconds"]) {
        Some(seconds) if seconds <= 6.0 * 3600.0 => "session",
        Some(seconds) if seconds <= 8.0 * 86400.0 => "weekly",
        Some(_) => "monthly",
        None => "limit",
    });
    let resets = number(&value["reset_at"]).map(|seconds| (seconds * 1000.0) as i64);
    Some(Meter::percent(label, used).resets(resets))
}
