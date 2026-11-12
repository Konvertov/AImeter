//! Codebuff: the session its command-line tool saved after `codebuff login`.

use super::*;

const BASE: &str = "https://www.codebuff.com";

fn saved_token() -> Option<String> {
    let file = home()?.join(".config").join("manicode").join("credentials.json");
    let login: Value = serde_json::from_str(&std::fs::read_to_string(file).ok()?).ok()?;
    login["default"]["authToken"]
        .as_str()
        .or_else(|| login["authToken"].as_str())
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let token = blocking(saved_token).await.ok_or(Fail::NotFound)?;
    let body = send_json(
        client
            .post(format!("{BASE}/api/v1/usage"))
            .bearer_auth(&token)
            .json(&serde_json::json!({ "fingerprintId": "aimeter-usage" })),
    )
    .await?;

    let mut data = Data::default();
    let used = number(&body["usage"]);
    let quota = number(&body["quota"]);
    let resets = moment_ms(&body["next_quota_reset"]);
    match (used, quota) {
        (Some(used), Some(quota)) if quota > 0.0 => {
            data.meters.push(Meter::of("credits", used, quota).resets(resets).primary());
        }
        _ => {
            if let Some(left) = number(&body["remainingBalance"]) {
                data.meters.push(Meter::text("credits", short(left)));
            }
        }
    }

    // Weekly limit and plan come from a second answer; optional.
    if let Ok(plan) =
        send_json(client.get(format!("{BASE}/api/user/subscription")).bearer_auth(&token)).await
    {
        let find = |key: &str| deep(&plan, key);
        if let (Some(used), Some(limit)) =
            (find("weeklyUsed").and_then(number), find("weeklyLimit").and_then(number))
        {
            if limit > 0.0 {
                data.meters.push(
                    Meter::of("weekly", used, limit)
                        .resets(find("weeklyResetsAt").and_then(moment_ms))
                        .primary(),
                );
            }
        }
        data.plan = find("tier")
            .or_else(|| find("displayName"))
            .and_then(Value::as_str)
            .map(title_case);
    }
    if data.meters.is_empty() {
        return Err(Fail::Other("unexpected response".into()));
    }
    Ok(data)
}
