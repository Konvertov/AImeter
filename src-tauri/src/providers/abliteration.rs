//! abliteration.ai: official API, needs the user's API key. Shows prepaid credits.

use super::*;
use crate::secrets;

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let info = super::info("abliteration").ok_or(Fail::NotFound)?;
    let key = blocking(move || secrets::api_key(info)).await.ok_or(Fail::NeedsKey)?;
    let body = send_json(client.get("https://api.abliteration.ai/v1/credits").bearer_auth(&key)).await?;

    // Both numbers are in dollars: how much was bought, how much is spent.
    let bought = number(&body["data"]["total_credits"]).filter(|n| *n >= 0.0);
    let spent = number(&body["data"]["total_usage"]).filter(|n| *n >= 0.0);
    let (Some(bought), Some(spent)) = (bought, spent) else {
        return Err(Fail::Other("unexpected response".into()));
    };

    let mut data = Data::default();
    if bought > 0.0 {
        data.meters.push(
            Meter::percent("credits", spent / bought * 100.0)
                .detail(format!("{} / {}", dollars(spent), dollars(bought)))
                .primary(),
        );
    }
    data.meters.push(Meter::text("balance", dollars((bought - spent).max(0.0))));
    Ok(data)
}
