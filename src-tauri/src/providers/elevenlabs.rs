//! ElevenLabs: documented API, needs the user's API key (with the
//! "User: read" permission). Shows the monthly credits of the plan.

use super::*;
use crate::secrets;

const URL: &str = "https://api.elevenlabs.io/v1/user/subscription";

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let info = super::info("elevenlabs").ok_or(Fail::NotFound)?;
    let key = blocking(move || secrets::api_key(info)).await.ok_or(Fail::NeedsKey)?;
    let response = client
        .get(URL)
        .header("xi-api-key", &key)
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(|e| Fail::Other(if e.is_timeout() { "timeout".into() } else { "network".into() }))?;
    if response.status().as_u16() == 401 {
        // A key made with limited permissions: say what it lacks.
        let answer: Value = response.json().await.unwrap_or(Value::Null);
        if answer["detail"]["status"].as_str() == Some("missing_permissions") {
            return Err(Fail::Other("key needs the User: read permission".into()));
        }
        return Err(Fail::Expired);
    }
    let body = send_json_response(response).await?;

    let used = number(&body["character_count"]).ok_or(Fail::Other("unexpected response".into()))?;
    let limit = number(&body["character_limit"]).unwrap_or(0.0);
    let resets = moment_ms(&body["next_character_count_reset_unix"]);

    let mut data = Data {
        plan: body["tier"].as_str().map(|tier| title_case(&tier.replace('_', " "))),
        ..Default::default()
    };
    data.meters.push(if limit > 0.0 {
        Meter::of("credits", used, limit).resets(resets).window(30 * DAY).primary()
    } else {
        Meter::text("credits", short(used))
    });
    Ok(data)
}
