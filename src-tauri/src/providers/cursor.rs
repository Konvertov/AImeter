//! Cursor: the editor keeps its sign-in in its own settings database.

use super::*;

const USAGE_URL: &str =
    "https://api2.cursor.sh/aiserver.v1.DashboardService/GetCurrentPeriodUsage";

struct Login {
    token: String,
    plan: Option<String>,
}

fn saved_login() -> Option<Login> {
    let file = dirs::config_dir()?
        .join("Cursor")
        .join("User")
        .join("globalStorage")
        .join("state.vscdb");
    if !file.exists() {
        return None;
    }
    let db = rusqlite::Connection::open_with_flags(
        &file,
        rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY | rusqlite::OpenFlags::SQLITE_OPEN_NO_MUTEX,
    )
    .ok()?;
    let _ = db.busy_timeout(std::time::Duration::from_secs(2));
    let read = |key: &str| -> Option<String> {
        db.query_row("SELECT value FROM ItemTable WHERE key = ?1", [key], |row| {
            row.get::<_, String>(0)
        })
        .ok()
        .map(|text| text.trim().trim_matches('"').to_string())
        .filter(|text| !text.is_empty())
    };
    Some(Login {
        token: read("cursorAuth/accessToken")?,
        plan: read("cursorAuth/stripeMembershipType"),
    })
}

pub async fn fetch(client: &reqwest::Client) -> Result<Data, Fail> {
    let login = blocking(saved_login).await.ok_or(Fail::NotFound)?;
    let body = send_json(
        client
            .post(USAGE_URL)
            .bearer_auth(&login.token)
            .header("Connect-Protocol-Version", "1")
            .json(&serde_json::json!({})),
    )
    .await?;

    let resets = number(&body["billingCycleEnd"]).map(|ms| ms as i64);
    let cycle = match (number(&body["billingCycleStart"]), resets) {
        (Some(start), Some(end)) => (end - start as i64) / 1000,
        _ => 0,
    };
    let plan = &body["planUsage"];
    let mut data = Data {
        plan: login.plan.as_deref().map(|p| title_case(&p.replace('_', " "))),
        meters: vec![],
        ..Default::default()
    };

    let limit = number(&plan["limit"]).unwrap_or(0.0);
    let spent = number(&plan["includedSpend"]).or_else(|| number(&plan["totalSpend"]));
    let total = number(&plan["totalPercentUsed"]).or_else(|| match spent {
        Some(spent) if limit > 0.0 => Some(spent / limit * 100.0),
        _ => None,
    });
    if let Some(total) = total {
        let mut meter = Meter::percent("plan_usage", total).resets(resets).window(cycle).primary();
        if let (Some(spent), true) = (spent, limit > 0.0) {
            meter = meter.detail(format!("{} / {}", dollars(spent / 100.0), dollars(limit / 100.0)));
        }
        data.meters.push(meter);
    }
    for (field, label) in [("autoPercentUsed", "auto_mode"), ("apiPercentUsed", "api_models")] {
        if let Some(used) = number(&plan[field]) {
            data.meters.push(Meter::percent(label, used).resets(resets));
        }
    }

    let extra = &body["spendLimitUsage"];
    let cap = number(&extra["individualLimit"]).unwrap_or(0.0);
    if cap > 0.0 {
        let used = number(&extra["individualUsed"]).unwrap_or(0.0);
        data.meters.push(
            Meter::percent("on_demand", used / cap * 100.0)
                .detail(format!("{} / {}", dollars(used / 100.0), dollars(cap / 100.0)))
                .resets(resets),
        );
    }
    if data.meters.is_empty() {
        return Err(Fail::Other("unexpected response".into()));
    }
    if paid(&data.plan) {
        data.billing = Billing::renews(resets);
    }
    Ok(data)
}
