//! Kiro: the editor already keeps the numbers it shows in its own state
//! file, so nothing is asked over the network.

use super::*;

pub async fn fetch() -> Result<Data, Fail> {
    let raw = blocking(|| editor_state("Kiro", "kiro.kiroAgent")).await.ok_or(Fail::NotFound)?;
    let state: Value = serde_json::from_str(&raw).map_err(|_| Fail::NotFound)?;
    let mut usage = state["kiro.resourceNotifications.usageState"].clone();
    if let Some(text) = usage.as_str() {
        usage = serde_json::from_str(text).unwrap_or(Value::Null);
    }
    let items = usage["usageBreakdowns"].as_array().ok_or(Fail::NotFound)?;

    let mut data = Data::default();
    for item in items {
        let resets = moment_ms(&item["resetDate"]);
        if let (Some(used), Some(limit)) = (number(&item["currentUsage"]), number(&item["usageLimit"])) {
            if limit > 0.0 {
                let meter = Meter::of("credits", used, limit).resets(resets);
                data.meters.push(if data.meters.is_empty() { meter.primary() } else { meter });
            }
        }
        let trial = &item["freeTrialUsage"];
        if let (Some(used), Some(limit)) = (number(&trial["currentUsage"]), number(&trial["usageLimit"])) {
            if limit > 0.0 {
                data.meters
                    .push(Meter::of("bonus_credits", used, limit).resets(moment_ms(&trial["expiryDate"])));
            }
        }
    }
    if data.meters.is_empty() {
        return Err(Fail::NotFound);
    }
    Ok(data)
}
