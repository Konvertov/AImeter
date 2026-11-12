//! Antigravity: while the app is open, a helper process of its own answers
//! on this computer with the quota of every model. We find that process,
//! find the port it listens on, and ask it. Nothing leaves the machine.

use super::*;
use std::time::Duration;

struct Helper {
    pid: u32,
    token: String,
}

/// The value of `--flag value` or `--flag=value` in a command line.
fn flag(args: &[String], name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    args.iter().enumerate().find_map(|(i, arg)| {
        if arg == name {
            args.get(i + 1).cloned()
        } else {
            arg.strip_prefix(&prefix).map(String::from)
        }
    })
}

fn find_helpers() -> Vec<Helper> {
    use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};
    let mut system = System::new();
    system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cmd(UpdateKind::Always),
    );
    let mut found = vec![];
    for (pid, process) in system.processes() {
        let name = process.name().to_string_lossy().to_lowercase();
        if !name.contains("language_server") {
            continue;
        }
        let args: Vec<String> =
            process.cmd().iter().map(|a| a.to_string_lossy().to_string()).collect();
        if !args.join(" ").to_lowercase().contains("antigravity") {
            continue;
        }
        if let Some(token) = flag(&args, "--csrf_token") {
            found.push(Helper { pid: pid.as_u32(), token });
        }
    }
    found
}

/// Ports the process listens on, lowest first.
fn listening_ports(pid: u32) -> Vec<u16> {
    let mut ports: Vec<u16> = vec![];
    let port_of = |address: &str| address.rsplit(':').next().and_then(|p| p.parse::<u16>().ok());
    if cfg!(windows) {
        if let Ok(out) = quiet_command("netstat").args(["-ano", "-p", "tcp"]).output() {
            for line in String::from_utf8_lossy(&out.stdout).lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 5 && parts[3] == "LISTENING" && parts[4] == pid.to_string() {
                    ports.extend(port_of(parts[1]));
                }
            }
        }
    } else if let Ok(out) = quiet_command("lsof")
        .args(["-nP", "-iTCP", "-sTCP:LISTEN", "-a", "-p", &pid.to_string()])
        .output()
    {
        for line in String::from_utf8_lossy(&out.stdout).lines().skip(1) {
            if let Some(address) = line.split_whitespace().rev().nth(1) {
                ports.extend(port_of(address));
            }
        }
    }
    ports.sort_unstable();
    ports.dedup();
    ports
}

pub async fn fetch() -> Result<Data, Fail> {
    let targets = blocking(|| {
        let list: Vec<(String, Vec<u16>)> = find_helpers()
            .into_iter()
            .map(|helper| (helper.token, listening_ports(helper.pid)))
            .collect();
        (!list.is_empty()).then_some(list)
    })
    .await
    .ok_or(Fail::NotFound)?;

    // The helper uses a certificate it made for itself, so the usual check is
    // off. This client is only ever pointed at 127.0.0.1.
    let local = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(|_| Fail::Other("local client".into()))?;
    let question = serde_json::json!({
        "metadata": {
            "ideName": "antigravity",
            "extensionName": "antigravity",
            "ideVersion": "unknown",
            "locale": "en"
        }
    });

    for (token, ports) in &targets {
        for port in ports {
            for scheme in ["https", "http"] {
                let url = format!(
                    "{scheme}://127.0.0.1:{port}/exa.language_server_pb.LanguageServerService/GetUserStatus"
                );
                let answer = local
                    .post(&url)
                    .header("Connect-Protocol-Version", "1")
                    .header("x-codeium-csrf-token", token)
                    .json(&question)
                    .send()
                    .await;
                let Ok(answer) = answer else { continue };
                if !answer.status().is_success() {
                    continue;
                }
                if let Ok(body) = answer.json::<Value>().await {
                    if let Some(data) = read_status(&body) {
                        return Ok(data);
                    }
                }
            }
        }
    }
    Err(Fail::Other("no answer from the app".into()))
}

fn read_status(body: &Value) -> Option<Data> {
    let status = &body["userStatus"];
    let models = status["cascadeModelConfigData"]["clientModelConfigs"].as_array()?;
    let mut data = Data {
        plan: status["planStatus"]["planInfo"]["planName"].as_str().map(String::from),
        meters: vec![],
        ..Default::default()
    };
    for model in models {
        let (Some(label), Some(left)) =
            (model["label"].as_str(), number(&model["quotaInfo"]["remainingFraction"]))
        else {
            continue;
        };
        data.meters.push(
            Meter::percent(label, (1.0 - left) * 100.0)
                .resets(moment_ms(&model["quotaInfo"]["resetTime"])),
        );
    }
    // The tray number follows the model that is closest to its limit.
    let tightest = data
        .meters
        .iter()
        .enumerate()
        .max_by(|a, b| a.1.used_percent.unwrap_or(0.0).total_cmp(&b.1.used_percent.unwrap_or(0.0)))
        .map(|(index, _)| index)?;
    data.meters[tightest].primary = true;
    Some(data)
}
