//! Windsurf: the editor caches its plan numbers in its own state file.
//! These are as fresh as the last time Windsurf itself was open.

use super::*;

pub async fn fetch() -> Result<Data, Fail> {
    let raw = blocking(|| editor_state("Windsurf", "windsurf.settings.cachedPlanInfo"))
        .await
        .ok_or(Fail::NotFound)?;
    let plan: Value = serde_json::from_str(&raw).map_err(|_| Fail::NotFound)?;
    let mut data = Data {
        plan: plan["planName"].as_str().map(String::from),
        meters: vec![],
        ..Default::default()
    };

    let quota = &plan["quotaUsage"];
    for (field, reset, label, length) in [
        ("dailyRemainingPercent", "dailyResetAtUnix", "daily", DAY),
        ("weeklyRemainingPercent", "weeklyResetAtUnix", "weekly", 7 * DAY),
    ] {
        if let Some(left) = number(&quota[field]) {
            data.meters
                .push(Meter::percent(label, 100.0 - left).resets(moment_ms(&quota[reset])).window(length).primary());
        }
    }

    let cycle_end = moment_ms(&plan["endTimestamp"]);
    let usage = &plan["usage"];
    for (total, used, label) in [
        ("messages", "usedMessages", "prompt_credits"),
        ("flowActions", "usedFlowActions", "flow_actions"),
        ("flexCredits", "usedFlexCredits", "flex_credits"),
    ] {
        if let (Some(total), Some(used)) = (number(&usage[total]), number(&usage[used])) {
            if total > 0.0 {
                let meter = Meter::of(label, used, total).resets(cycle_end);
                data.meters.push(if data.meters.is_empty() { meter.primary() } else { meter });
            }
        }
    }
    if data.meters.is_empty() {
        return Err(Fail::NotFound);
    }
    Ok(data)
}
