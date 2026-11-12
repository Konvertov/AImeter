//! Gemini (the app at gemini.google.com): the 5-hour and weekly limits of
//! the subscription. Google shows them only to a signed-in browser session,
//! so the user signs in once in a window AImeter opens (see `signin.rs`);
//! the session from that window is kept in the system password store.

use super::*;
use crate::secrets;

pub const SITE: &str = "https://gemini.google.com";
const QUERY_URL: &str =
    "https://gemini.google.com/_/BardChatUi/graphql/schemas/GEMINI_WEB_GRAPHQL/executeQuery";
/// Identifies the "usage page" question on Google's side.
const QUERY_ID: &str = "2/dWbCvkVB0X16r9NPnxifeOAAqdCqMf0xeWvau9yPy7g=";
const QUERY_INPUT: &str = r#"{"g1Request":{"entrypoints":["GEMINI_USAGE_SETTINGS_UPGRADE_BANNER"]},"workspaceRequest":{"recommendationsForSurfaceRequests":[]}}"#;

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let session = blocking(|| secrets::stored("gemini")).await.ok_or(Fail::NeedsKey)?;
    ask(client, &session).await
}

/// Ask Google for the usage numbers with the given browser session.
pub async fn ask(client: &reqwest::Client, session: &str) -> Result<Data, Fail> {
    let response = client
        .get(QUERY_URL)
        .query(&[("query_signature", QUERY_ID), ("variables", QUERY_INPUT)])
        .header("Accept", "application/json")
        .header("Referer", format!("{SITE}/usage"))
        .header("Cookie", session)
        .send()
        .await
        .map_err(|e| Fail::Other(if e.is_timeout() { "timeout".into() } else { "network".into() }))?;
    let code = response.status().as_u16();
    let text = response.text().await.unwrap_or_default();
    let text = text.trim_start();
    // A sign-in page instead of data means the session is no longer valid.
    if code == 401 || code == 403 || (code == 200 && text.starts_with('<')) {
        return Err(Fail::Expired);
    }
    if code == 429 {
        return Err(Fail::RateLimited);
    }
    if !(200..300).contains(&code) {
        return Err(Fail::Other(format!("HTTP {code}")));
    }
    // Google sometimes puts a guard line before the JSON.
    let json_start = text.find('{').ok_or(Fail::Other("unexpected response".into()))?;
    let body: Value =
        serde_json::from_str(&text[json_start..]).map_err(|_| Fail::Other("unexpected response".into()))?;
    let usage = deep(&body, "getUsageInfo").ok_or(Fail::Expired)?;
    let pools = usage["pools"].as_array().ok_or(Fail::Other("unexpected response".into()))?;

    let mut data = Data::default();
    data.plan = match usage["subscriptionTier"].as_str() {
        Some("TIER1") => Some("Pro".into()),
        _ => None,
    };
    for (kind, label, length) in
        [("GXU_FIVE_HOURLY", "session", 5 * HOUR), ("GXU_WEEKLY", "weekly", 7 * DAY)]
    {
        let Some(pool) = pools.iter().find(|pool| pool["poolType"].as_str() == Some(kind)) else {
            continue;
        };
        let Some(share) = number(&pool["percentageUsed"]) else {
            continue;
        };
        let refresh = &pool["gxu"]["windowRefreshesAt"];
        let resets = moment_ms(refresh).or_else(|| moment_ms(&refresh["seconds"]));
        data.meters
            .push(Meter::percent(label, share * 100.0).resets(resets).window(length).primary());
    }
    // Signed in, but this account has no such limits: still a valid answer.
    Ok(data)
}
