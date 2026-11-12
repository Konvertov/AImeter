//! GitHub Copilot: uses the sign-in of the GitHub command-line tool (`gh`).
//! The token is read each time and never stored by AImeter.

use super::*;
use crate::secrets;
use base64::Engine;

const USAGE_URL: &str = "https://api.github.com/copilot_internal/user";

fn saved_login() -> Option<String> {
    // Newer gh versions keep the token in the system password store.
    let stored = if cfg!(windows) {
        secrets::system_secret("gh:github.com:")
    } else {
        secrets::system_secret("gh:github.com")
    };
    if let Some(value) = stored {
        // The store may hold the token wrapped in base64 with a marker in front.
        if let Some(encoded) = value.strip_prefix("go-keyring-base64:") {
            let bytes = base64::engine::general_purpose::STANDARD.decode(encoded).ok()?;
            return String::from_utf8(bytes).ok();
        }
        return Some(value);
    }
    // Older versions wrote it to a settings file.
    let folder = if cfg!(windows) {
        dirs::config_dir()?.join("GitHub CLI")
    } else {
        dirs::home_dir()?.join(".config").join("gh")
    };
    let text = std::fs::read_to_string(folder.join("hosts.yml")).ok()?;
    let mut inside_github = false;
    for line in text.lines() {
        if !line.starts_with(' ') && !line.starts_with('\t') {
            inside_github = line.trim_end_matches(':').trim() == "github.com";
        } else if inside_github {
            if let Some(token) = line.trim().strip_prefix("oauth_token:") {
                let token = token.trim().trim_matches('"');
                if !token.is_empty() {
                    return Some(token.to_string());
                }
            }
        }
    }
    None
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let token = blocking(saved_login).await.ok_or(Fail::NotFound)?;
    let body = send_json(
        client
            .get(USAGE_URL)
            .header("Authorization", format!("token {token}"))
            .header("Editor-Version", "vscode/1.96.2")
            .header("Editor-Plugin-Version", "copilot-chat/0.26.7")
            .header("User-Agent", "GitHubCopilotChat/0.26.7")
            .header("X-Github-Api-Version", "2025-04-01"),
    )
    .await?;

    let mut data = Data {
        plan: body["copilot_plan"].as_str().map(|p| title_case(&p.replace('_', " "))),
        meters: vec![],
    };

    // Paid plans: each quota reports what is left.
    let resets = body["quota_reset_date"].as_str().and_then(iso_ms);
    let quotas = &body["quota_snapshots"];
    for (field, label, primary) in [
        ("premium_interactions", "premium", true),
        ("chat", "chat", false),
        ("completions", "completions", false),
    ] {
        let quota = &quotas[field];
        if quota["unlimited"].as_bool() == Some(true) {
            continue;
        }
        if let Some(left) = number(&quota["percent_remaining"]) {
            let mut meter = Meter::percent(label, 100.0 - left).resets(resets);
            if let (Some(left), Some(total)) = (number(&quota["remaining"]), number(&quota["entitlement"])) {
                meter = meter.detail(format!("{} / {}", (total - left).max(0.0).round(), total.round()));
            }
            data.meters.push(if primary { meter.primary() } else { meter });
        }
    }

    // Free plan: counts of what is left out of a monthly amount.
    let free_resets = body["limited_user_reset_date"].as_str().and_then(iso_ms);
    for (field, label) in [("chat", "chat"), ("completions", "completions")] {
        let left = number(&body["limited_user_quotas"][field]);
        let total = number(&body["monthly_quotas"][field]);
        if let (Some(left), Some(total)) = (left, total) {
            if total > 0.0 {
                data.meters.push(
                    Meter::percent(label, (total - left) / total * 100.0)
                        .detail(format!("{} / {}", (total - left).max(0.0).round(), total.round()))
                        .resets(free_resets)
                        .primary(),
                );
            }
        }
    }

    if data.meters.is_empty() {
        // Signed in to GitHub, but this account has no Copilot numbers to show.
        return Err(Fail::NotFound);
    }
    Ok(data)
}
