//! Grok: the sign-in the Grok command-line tool keeps in `~/.grok/auth.json`.

use super::*;

const BASE: &str = "https://cli-chat-proxy.grok.com/v1";

fn saved_token() -> Option<String> {
    let text = std::fs::read_to_string(home()?.join(".grok").join("auth.json")).ok()?;
    let login: Value = serde_json::from_str(&text).ok()?;
    // The file holds one entry per sign-in; each entry carries its token as "key".
    let from = |entry: &Value| {
        entry["key"]
            .as_str()
            .map(|t| t.trim().to_string())
            .filter(|t| !t.is_empty())
    };
    from(&login).or_else(|| login.as_object()?.values().find_map(from))
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let token = blocking(saved_token).await.ok_or(Fail::NotFound)?;
    let ask = |path: &str| {
        client
            .get(format!("{BASE}{path}"))
            .bearer_auth(&token)
            .header("X-XAI-Token-Auth", "xai-grok-cli")
    };
    let body = send_json(ask("/billing")).await?;
    let config = &body["config"];
    let used = number(&config["used"]["val"]).ok_or(Fail::Other("unexpected response".into()))?;
    let limit = number(&config["monthlyLimit"]["val"]).unwrap_or(0.0);
    let resets = moment_ms(&config["billingPeriodEnd"]);
    let cycle = match (moment_ms(&config["billingPeriodStart"]), resets) {
        (Some(start), Some(end)) => (end - start) / 1000,
        _ => 0,
    };

    let mut data = Data::default();
    if limit > 0.0 {
        data.meters.push(Meter::of("credits", used, limit).resets(resets).window(cycle).primary());
    } else {
        data.meters.push(Meter::text("credits", short(used)));
    }
    if let Some(cap) = number(&config["onDemandCap"]["val"]).filter(|cap| *cap > 0.0) {
        data.meters.push(Meter::text("pay_as_you_go", short(cap)));
    }
    // The plan name lives in another answer; it is fine if that one fails.
    if let Ok(settings) = send_json(ask("/settings")).await {
        data.plan = deep(&settings, "subscription_tier_display")
            .and_then(Value::as_str)
            .map(String::from);
    }
    Ok(data)
}
