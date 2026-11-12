//! DeepSeek: documented API, needs the user's API key. Shows the balance.

use super::*;
use crate::secrets;

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let info = super::info("deepseek").ok_or(Fail::NotFound)?;
    let key = blocking(move || secrets::api_key(info)).await.ok_or(Fail::NeedsKey)?;
    let body = send_json(client.get("https://api.deepseek.com/user/balance").bearer_auth(&key)).await?;

    let mut data = Data::default();
    if let Some(balances) = body["balance_infos"].as_array() {
        for entry in balances {
            let amount = number(&entry["total_balance"]).unwrap_or(0.0);
            let sign = match entry["currency"].as_str() {
                Some("CNY") => "¥",
                _ => "$",
            };
            data.meters.push(Meter::text("balance", format!("{sign}{amount:.2}")));
        }
    }
    if data.meters.is_empty() {
        data.meters.push(Meter::text("balance", "$0.00".into()));
    }
    Ok(data)
}
