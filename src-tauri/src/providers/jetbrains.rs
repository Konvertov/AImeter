//! JetBrains AI Assistant: every JetBrains IDE writes its AI quota into a
//! small settings file. We read the freshest one; nothing goes over the network.

use super::*;

/// Pull the JSON kept in `<option name="NAME" value="...">` out of the file.
fn option_json(xml: &str, name: &str) -> Option<Value> {
    let at = xml.find(&format!("name=\"{name}\""))?;
    let tag_start = xml[..at].rfind('<')?;
    let tag_end = at + xml[at..].find('>')?;
    let tag = &xml[tag_start..tag_end];
    let value_at = tag.find("value=\"")? + "value=\"".len();
    let value_end = value_at + tag[value_at..].find('"')?;
    let text = tag[value_at..value_end]
        .replace("&quot;", "\"")
        .replace("&#10;", "\n")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&apos;", "'")
        .replace("&amp;", "&");
    serde_json::from_str(&text).ok()
}

struct Quota {
    used: f64,
    limit: f64,
    until: Option<i64>,
    refill: Option<i64>,
}

fn read_quota(file: &std::path::Path) -> Option<Quota> {
    let xml = std::fs::read_to_string(file).ok()?;
    let info = option_json(&xml, "quotaInfo")?;
    let sum = |field: &str| -> Option<f64> {
        number(&info[field]).or_else(|| {
            let plan = number(&info["tariffQuota"][field]);
            let top_up = number(&info["topUpQuota"][field]);
            (plan.is_some() || top_up.is_some()).then(|| plan.unwrap_or(0.0) + top_up.unwrap_or(0.0))
        })
    };
    let limit = sum("maximum")?;
    let used = match sum("current") {
        Some(used) => used,
        None => limit - sum("available")?,
    };
    let refill = option_json(&xml, "nextRefill").and_then(|next| moment_ms(&next["next"]));
    Some(Quota { used, limit, until: moment_ms(&info["until"]), refill })
}

fn freshest() -> Option<Quota> {
    let base = dirs::config_dir()?.join("JetBrains");
    let mut best: Option<Quota> = None;
    for entry in std::fs::read_dir(base).ok()?.flatten() {
        let file = entry.path().join("options").join("AIAssistantQuotaManager2.xml");
        if let Some(quota) = read_quota(&file) {
            if quota.limit > 0.0 && best.as_ref().is_none_or(|b| quota.until > b.until) {
                best = Some(quota);
            }
        }
    }
    best
}

pub async fn fetch() -> Result<Data, Fail> {
    let quota = blocking(freshest).await.ok_or(Fail::NotFound)?;
    let mut data = Data::default();
    data.meters.push(
        Meter::percent("quota", quota.used / quota.limit * 100.0)
            .resets(quota.refill.or(quota.until))
            .primary(),
    );
    Ok(data)
}
