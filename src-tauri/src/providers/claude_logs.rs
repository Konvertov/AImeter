//! Token counts from the conversation logs Claude Code keeps on this
//! computer (`projects/**/*.jsonl`). Read-only, nothing leaves the machine.

use chrono::{Datelike, Local};
use serde::Serialize;
use serde_json::Value;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{Duration, SystemTime};

const DAYS: i32 = 30;

#[derive(Serialize, Clone, Debug, Default)]
pub struct Stats {
    pub today_tokens: f64,
    pub month_tokens: f64,
    /// One number per day, oldest first, today last.
    pub days: Vec<f64>,
    pub top_model: Option<String>,
    /// What these tokens would cost at public API prices, in dollars.
    pub today_cost: f64,
    pub month_cost: f64,
    /// True when some tokens came from models with no known price.
    pub cost_partial: bool,
}

struct Row {
    /// Same answer can be written into several files; this tells copies apart.
    key: u64,
    day: i32,
    model: String,
    tokens: f64,
    /// None when the model's price is not known.
    cost: Option<f64>,
}

struct Parsed {
    modified: SystemTime,
    size: u64,
    rows: Vec<Row>,
}

/// Files that did not change since the last look are not read again.
static SEEN: Mutex<Option<HashMap<PathBuf, Parsed>>> = Mutex::new(None);

fn log_files(folder: &Path, depth: u8, newer_than: SystemTime, out: &mut Vec<(PathBuf, SystemTime, u64)>) {
    let Ok(entries) = std::fs::read_dir(folder) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = entry.metadata() else { continue };
        if meta.is_dir() {
            if depth > 0 {
                log_files(&path, depth - 1, newer_than, out);
            }
        } else if path.extension().is_some_and(|ext| ext == "jsonl") {
            if let Ok(modified) = meta.modified() {
                if modified >= newer_than {
                    out.push((path, modified, meta.len()));
                }
            }
        }
    }
}

fn read_rows(file: &Path) -> Vec<Row> {
    let Ok(handle) = std::fs::File::open(file) else {
        return vec![];
    };
    let mut rows = vec![];
    for (index, line) in BufReader::new(handle).lines().map_while(Result::ok).enumerate() {
        if !line.contains("\"usage\"") {
            continue;
        }
        let Ok(entry) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        let message = &entry["message"];
        let usage = &message["usage"];
        let count = |field: &str| usage[field].as_f64().unwrap_or(0.0);
        let tokens = count("input_tokens")
            + count("output_tokens")
            + count("cache_creation_input_tokens")
            + count("cache_read_input_tokens");
        let model = message["model"].as_str().unwrap_or("");
        if tokens <= 0.0 || model.is_empty() || model.starts_with('<') {
            continue;
        }
        let Some(moment) = entry["timestamp"]
            .as_str()
            .and_then(|text| chrono::DateTime::parse_from_rfc3339(text).ok())
        else {
            continue;
        };
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        match message["id"].as_str() {
            Some(id) => (id, entry["requestId"].as_str().unwrap_or("")).hash(&mut hasher),
            None => (file, index).hash(&mut hasher),
        }
        let cost = super::prices::for_model(model).map(|price| {
            let written = count("cache_creation_input_tokens");
            let hour = usage["cache_creation"]["ephemeral_1h_input_tokens"].as_f64().unwrap_or(0.0).min(written);
            let write_units = (written - hour) * super::prices::CACHE_WRITE_5M + hour * super::prices::CACHE_WRITE_1H;
            ((count("input_tokens") + write_units) * price.input
                + count("output_tokens") * price.output
                + count("cache_read_input_tokens") * price.cache_read)
                / 1_000_000.0
        });
        rows.push(Row {
            cost,
            key: hasher.finish(),
            day: moment.with_timezone(&Local).date_naive().num_days_from_ce(),
            model: model.to_string(),
            tokens,
        });
    }
    rows
}

/// "claude-opus-4-5-20251101" -> "opus-4-5"
fn short_model(id: &str) -> String {
    let name = id.strip_prefix("claude-").unwrap_or(id);
    match name.rsplit_once('-') {
        Some((head, tail)) if tail.len() == 8 && tail.chars().all(|c| c.is_ascii_digit()) => head.to_string(),
        _ => name.to_string(),
    }
}

pub fn collect() -> Option<Stats> {
    let mut roots = vec![];
    if let Some(custom) = std::env::var_os("CLAUDE_CONFIG_DIR") {
        roots.push(PathBuf::from(custom));
    }
    if let Some(home) = dirs::home_dir() {
        roots.push(home.join(".claude"));
        roots.push(home.join(".config").join("claude"));
    }
    let newer_than = SystemTime::now() - Duration::from_secs((DAYS as u64 + 1) * 86400);
    let mut files = vec![];
    for root in roots {
        log_files(&root.join("projects"), 4, newer_than, &mut files);
    }
    if files.is_empty() {
        return None;
    }

    let mut guard = SEEN.lock().ok()?;
    let seen = guard.get_or_insert_with(HashMap::new);
    let current: HashSet<&PathBuf> = files.iter().map(|(path, _, _)| path).collect();
    seen.retain(|path, _| current.contains(path));
    for (path, modified, size) in &files {
        let fresh = seen.get(path).is_some_and(|p| p.modified == *modified && p.size == *size);
        if !fresh {
            seen.insert(path.clone(), Parsed { modified: *modified, size: *size, rows: read_rows(path) });
        }
    }

    let today = Local::now().date_naive().num_days_from_ce();
    let mut days = vec![0.0; DAYS as usize];
    let (mut today_cost, mut month_cost, mut cost_partial) = (0.0, 0.0, false);
    let mut by_model: HashMap<String, f64> = HashMap::new();
    let mut counted: HashSet<u64> = HashSet::new();
    for row in seen.values().flat_map(|parsed| parsed.rows.iter()) {
        let age = today - row.day;
        if !(0..DAYS).contains(&age) || !counted.insert(row.key) {
            continue;
        }
        days[(DAYS - 1 - age) as usize] += row.tokens;
        match row.cost {
            Some(cost) => {
                month_cost += cost;
                if age == 0 {
                    today_cost += cost;
                }
            }
            None => cost_partial = true,
        }
        *by_model.entry(short_model(&row.model)).or_default() += row.tokens;
    }
    let month_tokens: f64 = days.iter().sum();
    if month_tokens <= 0.0 {
        return None;
    }
    Some(Stats {
        today_tokens: days[DAYS as usize - 1],
        month_tokens,
        days,
        today_cost,
        month_cost,
        cost_partial,
        top_model: by_model.into_iter().max_by(|a, b| a.1.total_cmp(&b.1)).map(|(model, _)| model),
    })
}
