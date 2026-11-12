//! Devin: the session saved by the Devin command-line tool or the Devin app.

use super::*;

const DEFAULT_SERVER: &str = "https://server.codeium.com";

struct Login {
    key: String,
    server: String,
}

/// `name = "value"` from a settings file.
fn setting(text: &str, name: &str) -> Option<String> {
    text.lines().find_map(|line| {
        let (left, right) = line.split_once('=')?;
        (left.trim() == name)
            .then(|| right.trim().trim_matches('"').trim_matches('\'').to_string())
            .filter(|value| !value.is_empty())
    })
}

/// The session is only ever sent to Devin's own servers.
fn trusted(server: &str) -> bool {
    server
        .strip_prefix("https://")
        .map(|rest| rest.split(['/', ':']).next().unwrap_or(""))
        .is_some_and(|host| {
            host == "server.codeium.com" || host.ends_with(".codeium.com") || host.ends_with(".devin.ai")
        })
}

fn saved_login() -> Option<Login> {
    let mut folders = vec![];
    if let Some(dir) = dirs::config_dir() {
        folders.push(dir.join("devin"));
    }
    if let Some(dir) = dirs::data_dir() {
        folders.push(dir.join("devin"));
    }
    if let Some(home) = home() {
        folders.push(home.join(".local").join("share").join("devin"));
    }
    for folder in folders {
        if let Ok(text) = std::fs::read_to_string(folder.join("credentials.toml")) {
            if let Some(key) = setting(&text, "windsurf_api_key") {
                let server = setting(&text, "api_server_url")
                    .filter(|url| trusted(url))
                    .unwrap_or_else(|| DEFAULT_SERVER.into());
                return Some(Login { key, server: server.trim_end_matches('/').to_string() });
            }
        }
    }
    let status = editor_state("Devin", "windsurfAuthStatus")?;
    let status: Value = serde_json::from_str(&status).ok()?;
    let key = status["apiKey"].as_str()?.trim().to_string();
    (!key.is_empty()).then(|| Login { key, server: DEFAULT_SERVER.into() })
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let login = blocking(saved_login).await.ok_or(Fail::NotFound)?;
    let body = send_json(
        client
            .post(format!(
                "{}/exa.seat_management_pb.SeatManagementService/GetUserStatus",
                login.server
            ))
            .header("Connect-Protocol-Version", "1")
            .json(&serde_json::json!({
                "metadata": {
                    "apiKey": login.key,
                    "ideName": "devin",
                    "ideVersion": "1.108.2",
                    "extensionName": "devin",
                    "extensionVersion": "1.108.2",
                    "locale": "en"
                }
            })),
    )
    .await?;

    let status = &body["userStatus"]["planStatus"];
    let info = &status["planInfo"];
    let hide_daily = info["hideDailyQuota"].as_bool() == Some(true);
    let mut data = Data {
        plan: info["planName"].as_str().map(String::from),
        meters: vec![],
        ..Default::default()
    };

    let daily_left = number(&status["dailyQuotaRemainingPercent"]);
    // When the daily figure is hidden, the service reports the weekly one in its place.
    let weekly_left =
        number(&status["weeklyQuotaRemainingPercent"]).or(if hide_daily { daily_left } else { None });
    if let Some(left) = weekly_left {
        data.meters.push(
            Meter::percent("weekly", 100.0 - left)
                .resets(moment_ms(&status["weeklyQuotaResetAtUnix"]))
                .window(7 * DAY)
                .primary(),
        );
    }
    if let (false, Some(left)) = (hide_daily, daily_left) {
        data.meters.push(
            Meter::percent("daily", 100.0 - left)
                .resets(moment_ms(&status["dailyQuotaResetAtUnix"]))
                .window(DAY)
                .primary(),
        );
    }
    if let Some(micros) = number(&status["overageBalanceMicros"]).filter(|m| *m > 0.0) {
        data.meters.push(Meter::text("extra_balance", dollars(micros / 1_000_000.0)));
    }
    if data.meters.is_empty() {
        return Err(Fail::Other("unexpected response".into()));
    }
    Ok(data)
}
