//! Amp: the key the Amp command-line tool saved after sign-in.

use super::*;

fn saved_key() -> Option<String> {
    let mut places = vec![];
    if let Some(home) = home() {
        places.push(home.join(".local").join("share").join("amp"));
    }
    if let Some(data) = dirs::data_dir() {
        places.push(data.join("amp"));
    }
    for folder in places {
        let Ok(text) = std::fs::read_to_string(folder.join("secrets.json")) else {
            continue;
        };
        let Ok(secrets) = serde_json::from_str::<Value>(&text) else {
            continue;
        };
        if let Some(key) = secrets["apiKey@https://ampcode.com/"].as_str() {
            if !key.trim().is_empty() {
                return Some(key.trim().to_string());
            }
        }
    }
    None
}

/// The dollar amount written right after `marker`, e.g. "$4.20" -> 4.2.
fn money_after(text: &str, marker: &str) -> Option<f64> {
    let rest = &text[text.find(marker)? + marker.len()..];
    let rest = rest.trim_start().trim_start_matches('+').trim_start_matches('$');
    let digits: String = rest
        .chars()
        .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
        .filter(|c| *c != ',')
        .collect();
    digits.trim_end_matches('.').parse().ok()
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let key = blocking(saved_key).await.ok_or(Fail::NotFound)?;
    let body = send_json(
        client
            .post("https://ampcode.com/api/internal")
            .bearer_auth(&key)
            .json(&serde_json::json!({ "method": "userDisplayBalanceInfo", "params": {} })),
    )
    .await?;
    // The service answers with ready-made sentences; the numbers are inside.
    let text = deep(&body, "displayText")
        .and_then(Value::as_str)
        .ok_or(Fail::Other("unexpected response".into()))?;

    let mut data = Data::default();
    for line in text.lines() {
        if let Some(free) = line.strip_prefix("Amp Free:") {
            let left = money_after(free, "");
            let total = free.find('/').and_then(|slash| money_after(&free[slash..], "/"));
            if let (Some(left), Some(total)) = (left, total) {
                if total > 0.0 {
                    let used = (total - left).max(0.0);
                    let mut meter = Meter::percent("free_tier", used / total * 100.0)
                        .detail(format!("{} / {}", dollars(used), dollars(total)))
                        .primary();
                    // It refills by the hour: full again after used / rate hours.
                    if let Some(rate) = money_after(free, "replenishes").filter(|r| *r > 0.0) {
                        if used > 0.0 {
                            meter = meter.resets(Some(now_ms() + (used / rate * 3_600_000.0) as i64));
                        }
                    }
                    data.meters.push(meter);
                    data.plan = Some("Free".into());
                }
            }
        } else if line.starts_with("Individual credits:") {
            if let Some(credits) = money_after(line, "Individual credits:") {
                data.meters.push(Meter::text("credits", dollars(credits)));
            }
        }
    }
    if data.meters.is_empty() {
        return Err(Fail::Other("unexpected response".into()));
    }
    Ok(data)
}
