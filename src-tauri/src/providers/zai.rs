//! Z.ai (GLM Coding Plan): needs the user's API key. The limits come from
//! the address the Z.ai site itself uses for its usage page; it is not part
//! of the documented API, so unknown fields are skipped, not guessed.

use super::*;
use crate::secrets;

/// International site first, then the mainland China one (keys differ).
const URLS: [&str; 2] = [
    "https://api.z.ai/api/monitor/usage/quota/limit",
    "https://open.bigmodel.cn/api/monitor/usage/quota/limit",
];

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let info = super::info("zai").ok_or(Fail::NotFound)?;
    let key = blocking(move || secrets::api_key(info)).await.ok_or(Fail::NeedsKey)?;

    let mut body = Value::Null;
    let mut last = Fail::Expired;
    for url in URLS {
        // The raw key goes in the header as is, without "Bearer".
        match send_json(client.get(url).header("Authorization", &key)).await {
            Ok(answer) if answer["data"].is_object() => {
                body = answer;
                break;
            }
            Ok(_) => last = Fail::Expired,
            Err(fail) => last = fail,
        }
    }
    let limits = body["data"]["limits"].as_array().ok_or(last)?;

    let mut data = Data {
        plan: body["data"]["level"].as_str().map(title_case),
        ..Default::default()
    };
    for item in limits {
        let resets = moment_ms(&item["nextResetTime"]);
        match item["type"].as_str() {
            Some("TOKENS_LIMIT") => {
                let Some(used) = number(&item["percentage"]) else { continue };
                let count = number(&item["number"]).unwrap_or(1.0).max(1.0) as i64;
                let (label, length) = match number(&item["unit"]).map(|u| u as i64) {
                    Some(3) => ("session", count * HOUR),
                    Some(6) => ("weekly", count * 7 * DAY),
                    _ => ("limit", 0),
                };
                data.meters.push(Meter::percent(label, used).resets(resets).window(length).primary());
            }
            Some("TIME_LIMIT") => {
                let total = number(&item["usage"]);
                let spent = number(&item["currentValue"]);
                let meter = match (spent, total) {
                    (Some(spent), Some(total)) if total > 0.0 => Meter::of("mcp_tools", spent, total),
                    _ => match number(&item["percentage"]) {
                        Some(used) => Meter::percent("mcp_tools", used),
                        None => continue,
                    },
                };
                data.meters.push(meter.resets(resets));
            }
            _ => {}
        }
    }
    if data.meters.is_empty() {
        return Err(Fail::Other("unexpected response".into()));
    }
    Ok(data)
}
