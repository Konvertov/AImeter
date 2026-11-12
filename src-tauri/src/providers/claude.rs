//! Claude: the sign-in Claude Code keeps on this computer.
//! We only read it. Renewing it is Claude Code's job: its renewal tokens
//! are single-use, so renewing from here could sign the user out.

use super::*;
use crate::secrets;

const USAGE_URL: &str = "https://api.anthropic.com/api/oauth/usage";

fn saved_login() -> Option<String> {
    if let Some(text) = secrets::system_secret("Claude Code-credentials") {
        return Some(text);
    }
    let folder = std::env::var_os("CLAUDE_CONFIG_DIR")
        .map(std::path::PathBuf::from)
        .or_else(|| dirs::home_dir().map(|home| home.join(".claude")))?;
    std::fs::read_to_string(folder.join(".credentials.json")).ok()
}

/// Claude Code writes the signed-in e-mail into its settings file.
fn account_email() -> Option<String> {
    let folder = std::env::var_os("CLAUDE_CONFIG_DIR")
        .map(std::path::PathBuf::from)
        .or_else(dirs::home_dir)?;
    let text = std::fs::read_to_string(folder.join(".claude.json")).ok()?;
    let settings: Value = serde_json::from_str(&text).ok()?;
    settings["oauthAccount"]["emailAddress"].as_str().map(String::from)
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let raw = blocking(saved_login).await.ok_or(Fail::NotFound)?;
    let login: Value = serde_json::from_str(&raw).map_err(|_| Fail::NotFound)?;
    let oauth = &login["claudeAiOauth"];
    let token = oauth["accessToken"].as_str().ok_or(Fail::NotFound)?;

    let body = send_json(
        client
            .get(USAGE_URL)
            .bearer_auth(token)
            .header("anthropic-beta", "oauth-2025-04-20"),
    )
    .await?;

    let mut data = Data {
        account: blocking(account_email).await,
        plan: oauth["subscriptionType"].as_str().map(title_case),
        meters: vec![],
        ..Default::default()
    };

    // Known windows first, in a fixed order; anything new the service adds
    // later still shows up under its raw name.
    let known = [
        ("five_hour", "session", true, 5 * HOUR),
        ("seven_day", "weekly", true, 7 * DAY),
        ("seven_day_opus", "weekly_opus", false, 7 * DAY),
        ("seven_day_sonnet", "weekly_sonnet", false, 7 * DAY),
    ];
    let mut seen: Vec<&str> = vec!["extra_usage"];
    for (field, label, primary, length) in known {
        seen.push(field);
        if let Some(meter) = window(&body[field], label).map(|m| m.window(length)) {
            data.meters.push(if primary { meter.primary() } else { meter });
        }
    }
    if let Some(fields) = body.as_object() {
        for (field, value) in fields {
            if !seen.contains(&field.as_str()) {
                if let Some(meter) = window(value, field) {
                    data.meters.push(meter);
                }
            }
        }
    }

    let extra = &body["extra_usage"];
    if extra["is_enabled"].as_bool() == Some(true) {
        let spent = number(&extra["used_credits"]).unwrap_or(0.0) / 100.0;
        let cap = number(&extra["monthly_limit"]).unwrap_or(0.0) / 100.0;
        data.meters.push(if cap > 0.0 {
            Meter::percent("extra_usage", spent / cap * 100.0)
                .detail(format!("{} / {}", dollars(spent), dollars(cap)))
        } else {
            Meter::text("extra_usage", dollars(spent))
        });
    }
    Ok(data)
}

fn window(value: &Value, label: &str) -> Option<Meter> {
    let used = number(&value["utilization"])?;
    let resets = value["resets_at"].as_str().and_then(iso_ms);
    Some(Meter::percent(label, used).resets(resets))
}
