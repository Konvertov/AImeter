//! OpenRouter: documented API, needs the user's API key.

use super::*;
use crate::secrets;

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let info = super::info("openrouter").ok_or(Fail::NotFound)?;
    let key = blocking(move || secrets::api_key(info)).await.ok_or(Fail::NeedsKey)?;

    let body = send_json(client.get("https://openrouter.ai/api/v1/key").bearer_auth(&key)).await?;
    let key_data = &body["data"];
    let mut data = Data::default();

    // Account balance. Some keys are not allowed to read it; that is fine.
    if let Ok(credits) =
        send_json(client.get("https://openrouter.ai/api/v1/credits").bearer_auth(&key)).await
    {
        let bought = number(&credits["data"]["total_credits"]);
        let spent = number(&credits["data"]["total_usage"]);
        if let (Some(bought), Some(spent)) = (bought, spent) {
            if bought > 0.0 {
                data.meters.push(
                    Meter::percent("credits", spent / bought * 100.0)
                        .detail(format!("{} / {}", dollars(spent), dollars(bought)))
                        .primary(),
                );
            }
        }
    }

    // Spending cap of this particular key, when it has one.
    if let Some(cap) = number(&key_data["limit"]).filter(|cap| *cap > 0.0) {
        let spent = match number(&key_data["limit_remaining"]) {
            Some(left) => cap - left,
            None => number(&key_data["usage"]).unwrap_or(0.0),
        };
        data.meters.push(
            Meter::percent("key_limit", spent / cap * 100.0)
                .detail(format!("{} / {}", dollars(spent.max(0.0)), dollars(cap))),
        );
    }
    for (field, label) in [("usage_daily", "today"), ("usage_monthly", "this_month")] {
        if let Some(spent) = number(&key_data[field]) {
            data.meters.push(Meter::text(label, dollars(spent)));
        }
    }
    if data.meters.is_empty() {
        if let Some(spent) = number(&key_data["usage"]) {
            data.meters.push(Meter::text("spent", dollars(spent)));
        }
    }
    Ok(data)
}
