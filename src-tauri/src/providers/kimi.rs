//! Kimi Code: the sign-in the Kimi command-line tool keeps on this computer.

use super::*;

fn saved_token() -> Option<String> {
    let file = home()?.join(".kimi").join("credentials").join("kimi-code.json");
    let login: Value = serde_json::from_str(&std::fs::read_to_string(file).ok()?).ok()?;
    login["access_token"]
        .as_str()
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
}

fn meter(label: &str, detail: &Value) -> Option<Meter> {
    let limit = number(&detail["limit"]).filter(|l| *l > 0.0)?;
    let left = number(&detail["remaining"])?;
    Some(Meter::percent(label, (limit - left) / limit * 100.0).resets(moment_ms(&detail["resetTime"])))
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let token = blocking(saved_token).await.ok_or(Fail::NotFound)?;
    let body = send_json(client.get("https://api.kimi.com/coding/v1/usages").bearer_auth(&token)).await?;

    let mut data = Data::default();
    if let Some(level) = deep(&body["user"], "level").and_then(Value::as_str) {
        data.plan = Some(title_case(&level.trim_start_matches("LEVEL_").to_lowercase()));
    }
    // Short windows (a few hours) first, then the overall allowance.
    if let Some(limits) = body["limits"].as_array() {
        for entry in limits {
            let minutes = match (number(&entry["window"]["duration"]), entry["window"]["timeUnit"].as_str()) {
                (Some(n), Some("TIME_UNIT_HOUR")) => n * 60.0,
                (Some(n), Some("TIME_UNIT_DAY")) => n * 1440.0,
                (Some(n), _) => n,
                _ => 0.0,
            };
            let label = if minutes <= 360.0 { "session" } else if minutes <= 1440.0 { "daily" } else { "limit" };
            if let Some(found) = meter(label, &entry["detail"]) {
                data.meters.push(found.primary());
            }
        }
    }
    if let Some(found) = meter("weekly", &body["usage"]) {
        data.meters.push(found.primary());
    }
    if data.meters.is_empty() {
        return Err(Fail::Other("unexpected response".into()));
    }
    Ok(data)
}
