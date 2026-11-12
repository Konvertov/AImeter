//! OpenCode Go: the key OpenCode saved after sign-in (in its login file,
//! or in its database in newer versions).

use super::*;

fn saved_key() -> Option<String> {
    let mut folders = vec![];
    if let Some(home) = home() {
        folders.push(home.join(".local").join("share").join("opencode"));
    }
    if let Some(data) = dirs::data_local_dir() {
        folders.push(data.join("opencode"));
    }
    for folder in &folders {
        // Newer versions: a row per sign-in inside each opencode*.db file.
        if let Ok(entries) = std::fs::read_dir(folder) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("opencode") && name.ends_with(".db") {
                    let key = sqlite_text(
                        &entry.path(),
                        "SELECT json_extract(value, '$.key') FROM credential WHERE integration_id = ?1",
                        "opencode-go",
                    );
                    if key.is_some() {
                        return key;
                    }
                }
            }
        }
        // Older versions: one login file.
        if let Ok(text) = std::fs::read_to_string(folder.join("auth.json")) {
            if let Ok(login) = serde_json::from_str::<Value>(&text) {
                if let Some(key) = login["opencode-go"]["key"].as_str() {
                    if !key.trim().is_empty() {
                        return Some(key.trim().to_string());
                    }
                }
            }
        }
    }
    None
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let key = blocking(saved_key).await.ok_or(Fail::NotFound)?;
    let body = send_json(client.get("https://opencode.ai/zen/go/v1/usage").bearer_auth(&key)).await?;
    let usage = &body["usage"];
    let mut data = Data::default();
    for (field, label, length) in [("rolling", "session", 5 * HOUR), ("weekly", "weekly", 7 * DAY), ("monthly", "monthly", 30 * DAY)] {
        if let Some(used) = number(&usage[field]["percent"]) {
            let meter = Meter::percent(label, used).resets(moment_ms(&usage[field]["resetsAt"])).window(length);
            data.meters.push(if label == "monthly" { meter } else { meter.primary() });
        }
    }
    if data.meters.is_empty() {
        return Err(Fail::Other("unexpected response".into()));
    }
    Ok(data)
}
