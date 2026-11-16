mod icon;
mod providers;
mod secrets;
mod signin;
mod updates;

use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
use std::sync::Mutex;
use std::time::Duration;

use providers::Snapshot;
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{AppHandle, Emitter, Manager, WindowEvent};
use tauri_plugin_positioner::{Position, WindowExt};

/// How often every service is asked again.
const REFRESH_EVERY: Duration = Duration::from_secs(5 * 60);
/// How long one service may take before the refresh goes on without it.
const SERVICE_LIMIT: Duration = Duration::from_secs(30);
/// How long a service is left alone after it asked us to slow down.
const SLOW_DOWN_MS: i64 = 20 * 60 * 1000;
/// Windows draws the panel larger than macOS; there the page is shown at
/// this share of its size.
#[cfg(windows)]
const PANEL_ZOOM: f64 = 0.9;

pub(crate) struct Shared {
    pub(crate) client: reqwest::Client,
    snapshots: Mutex<Vec<Snapshot>>,
    /// Service id -> time (ms) before which it must not be asked again.
    quiet_until: Mutex<HashMap<String, i64>>,
    busy: tokio::sync::Mutex<()>,
    hidden_at: AtomicI64,
    /// The user prefers "how much is left" over "how much is spent".
    show_left: AtomicBool,
    /// When a Gemini session was last renewed quietly (ms), to not do it in a loop.
    renewed_at: AtomicI64,
    /// Service whose limit is shown next to the icon; None = the tightest of all.
    tray_service: Mutex<Option<String>>,
    /// The user may switch automatic updates off in Settings.
    pub(crate) auto_update: AtomicBool,
    /// Services the user switched off; they are neither asked nor shown.
    hidden: Mutex<HashSet<String>>,
    /// The percent the icon was last drawn for (-1 = empty bar, -2 = not drawn yet).
    /// Only Windows draws the icon; macOS shows text instead.
    #[cfg_attr(target_os = "macos", allow(dead_code))]
    icon_percent: AtomicI64,
}

pub(crate) async fn refresh(app: AppHandle) {
    let shared = app.state::<Shared>();
    let Ok(_guard) = shared.busy.try_lock() else {
        return; // a refresh is already running
    };

    let now = providers::now_ms();
    let previous = shared.snapshots.lock().unwrap().clone();
    let quiet = shared.quiet_until.lock().unwrap().clone();
    let hidden = shared.hidden.lock().unwrap().clone();

    let mut jobs = Vec::new();
    for info in providers::ALL {
        if hidden.contains(info.id) || quiet.get(info.id).is_some_and(|until| *until > now) {
            continue;
        }
        let client = shared.client.clone();
        // One service waiting (say, on a system permission window) must not
        // hold back the others: after the limit it is reported as "no answer".
        jobs.push(tauri::async_runtime::spawn(async move {
            match tokio::time::timeout(SERVICE_LIMIT, providers::snapshot(info, &client)).await {
                Ok(snapshot) => snapshot,
                Err(_) => providers::timed_out(info),
            }
        }));
    }

    let mut fresh: Vec<Snapshot> = Vec::new();
    for job in jobs {
        if let Ok(snapshot) = job.await {
            fresh.push(snapshot);
        }
    }

    let mut result = Vec::new();
    for info in providers::ALL {
        if hidden.contains(info.id) {
            result.push(providers::hidden(info));
            continue;
        }
        let old = previous.iter().find(|s| s.id == info.id && s.state != "hidden").cloned();
        match fresh.iter().find(|s| s.id == info.id).cloned() {
            Some(mut new) => {
                if new.state == "rate_limited" {
                    shared
                        .quiet_until
                        .lock()
                        .unwrap()
                        .insert(info.id.to_string(), now + SLOW_DOWN_MS);
                }
                // A hiccup should not wipe numbers we already had: keep the
                // last good ones on screen, marked with the new state.
                if matches!(new.state.as_str(), "rate_limited" | "error" | "expired") {
                    if let Some(old) = old.filter(|o| !o.meters.is_empty()) {
                        new.meters = old.meters;
                        new.plan = old.plan;
                        new.account = old.account;
                        new.fetched_at = old.fetched_at;
                    }
                }
                result.push(new);
            }
            None => result.extend(old),
        }
    }

    // Preview aid: AIMETER_DEMO=3 shows three services with the numbers of the
    // first working one (under the names of other services), to see how the
    // panel looks with several. Off unless asked for.
    if let Some(copies) = std::env::var("AIMETER_DEMO").ok().and_then(|n| n.parse::<usize>().ok()) {
        if let Some(sample) = result.iter().find(|s| s.state == "ok" && !s.meters.is_empty()).cloned() {
            let stand_ins: Vec<&providers::Info> = providers::ALL
                .iter()
                .filter(|info| info.id != sample.id && info.key_env.is_none() && !info.connect)
                .take(copies.clamp(1, 10) - 1)
                .collect();
            for info in stand_ins {
                if let Some(slot) = result.iter_mut().find(|s| s.id == info.id) {
                    let mut copy = sample.clone();
                    copy.id = info.id.to_string();
                    copy.name = info.name.to_string();
                    *slot = copy;
                }
            }
        }
    }

    // An expired Gemini session can often be renewed without the user: the
    // built-in browser still remembers the sign-in. Try at most once an hour.
    let gemini_expired = result.iter().any(|s| s.id == "gemini" && s.state == "expired");
    if gemini_expired && now - shared.renewed_at.load(Ordering::Relaxed) > 3_600_000 {
        shared.renewed_at.store(now, Ordering::Relaxed);
        let _ = signin::open_gemini(&app, true);
    }

    *shared.snapshots.lock().unwrap() = result.clone();
    update_tray(&app, &result);
    let _ = app.emit("snapshots", result);
}

/// The number next to the icon: the tightest main limit across services.
fn update_tray(app: &AppHandle, snapshots: &[Snapshot]) {
    // Each service is represented by its first main limit: the session (the
    // 5-hour window) where it has one. Weekly limits stay in the panel.
    let tightest_of = |only: Option<&str>| {
        snapshots
            .iter()
            .filter(|s| only.is_none_or(|id| s.id == id))
            .filter_map(|s| {
                s.meters
                    .iter()
                    .find(|m| m.primary && m.used_percent.is_some())
                    .and_then(|m| m.used_percent)
                    .map(|p| (s, p))
            })
            .max_by(|a, b| a.1.total_cmp(&b.1))
    };
    // The chosen service; if it has nothing to show right now, the tightest of all.
    let chosen = app.state::<Shared>().tray_service.lock().unwrap().clone();
    let tightest = chosen
        .as_deref()
        .and_then(|id| tightest_of(Some(id)))
        .or_else(|| tightest_of(None));
    if let Some(tray) = app.tray_by_id("main") {
        let shared = app.state::<Shared>();
        let shown = tightest.map(|(service, used)| {
            let percent = if shared.show_left.load(Ordering::Relaxed) { 100.0 - used } else { used };
            (service, percent)
        });
        match shown {
            Some((service, percent)) => {
                // In the macOS menu bar the app is "AI 84%": bold letters (a picture)
                // followed by the number as text.
                let _ = tray.set_title(Some(format!("{percent:.0}%")));
                let _ = tray.set_tooltip(Some(format!("AI Meter: {} {percent:.0}%", service.name)));
            }
            None => {
                let _ = tray.set_title(None::<String>);
                let _ = tray.set_tooltip(Some("AI Meter"));
            }
        }
        // Windows cannot show text next to a tray icon, so there the picture
        // carries the level: Kvadrik's belly cells show what is left.
        // Redraw only when it changed.
        #[cfg(not(target_os = "macos"))]
        {
            let left = tightest.map(|(_, used)| 100.0 - used);
            let level = left.map(|left| left.round() as i64).unwrap_or(-1);
            if shared.icon_percent.swap(level, Ordering::Relaxed) != level {
                let _ = tray.set_icon(Some(icon::tray_character(left)));
            }
        }
    }
}

fn panel_spot() -> Position {
    if cfg!(target_os = "macos") {
        Position::TrayBottomCenter
    } else {
        Position::TrayCenter
    }
}

/// The panel is as tall as its content; the UI reports the height it needs.
#[tauri::command]
fn fit_panel(app: AppHandle, height: f64) {
    let Some(panel) = app.get_webview_window("main") else {
        return;
    };
    let height = height.clamp(150.0, 640.0);
    let _ = panel.set_size(tauri::LogicalSize::new(360.0, height));
    // Keep it attached to the icon after it grew or shrank.
    if panel.is_visible().unwrap_or(false) {
        let _ = panel.move_window(panel_spot());
    }
}

/// Open the panel (never close it), e.g. when the app is started again.
/// Next to the tray icon if its place is known yet, else above the clock.
#[cfg(windows)]
fn show_panel(app: &AppHandle) {
    let Some(panel) = app.get_webview_window("main") else {
        return;
    };
    if panel.move_window(panel_spot()).is_err() {
        let _ = panel.move_window(Position::BottomRight);
    }
    let _ = panel.show();
    let _ = panel.set_focus();
}

fn toggle_panel(app: &AppHandle) {
    let Some(panel) = app.get_webview_window("main") else {
        return;
    };
    if panel.is_visible().unwrap_or(false) {
        let _ = panel.hide();
        return;
    }
    // Clicking the icon while the panel is open first takes focus away
    // (which hides it); do not reopen it on that same click.
    let hidden_at = app.state::<Shared>().hidden_at.load(Ordering::Relaxed);
    if providers::now_ms() - hidden_at < 250 {
        return;
    }
    let _ = panel.move_window(panel_spot());
    let _ = panel.show();
    let _ = panel.set_focus();
}

/// The panel tells which way the user wants the numbers shown.
#[tauri::command]
fn set_show_left(app: AppHandle, left: bool) {
    let shared = app.state::<Shared>();
    shared.show_left.store(left, Ordering::Relaxed);
    let snapshots = shared.snapshots.lock().unwrap().clone();
    update_tray(&app, &snapshots);
}

/// Which service the number next to the icon follows; empty = the tightest.
#[tauri::command]
fn set_tray_service(app: AppHandle, id: String) {
    let shared = app.state::<Shared>();
    *shared.tray_service.lock().unwrap() = providers::info(&id).map(|info| info.id.to_string());
    let snapshots = shared.snapshots.lock().unwrap().clone();
    update_tray(&app, &snapshots);
}

#[tauri::command]
fn set_auto_update(app: AppHandle, on: bool) {
    app.state::<Shared>().auto_update.store(on, Ordering::Relaxed);
}

/// The panel tells which services the user switched off.
#[tauri::command]
async fn set_hidden_services(app: AppHandle, ids: Vec<String>) {
    {
        let shared = app.state::<Shared>();
        *shared.hidden.lock().unwrap() = ids.into_iter().collect();
        // Let a refresh that is already running finish, then apply the change.
        let _wait = shared.busy.lock().await;
    }
    refresh(app).await;
}

#[tauri::command]
fn get_snapshots(shared: tauri::State<Shared>) -> Vec<Snapshot> {
    shared.snapshots.lock().unwrap().clone()
}

#[tauri::command]
async fn refresh_now(app: AppHandle) {
    refresh(app).await;
}

#[tauri::command]
async fn set_api_key(app: AppHandle, id: String, key: String) -> Result<(), String> {
    if !providers::info(&id).is_some_and(|info| info.key_env.is_some()) {
        return Err("unknown service".into());
    }
    let service = id.clone();
    tauri::async_runtime::spawn_blocking(move || secrets::save_api_key(&service, &key))
        .await
        .map_err(|e| e.to_string())??;
    app.state::<Shared>().quiet_until.lock().unwrap().remove(&id);
    refresh(app).await;
    Ok(())
}

#[tauri::command]
fn connect_service(app: AppHandle, id: String) -> Result<(), String> {
    if !signin::can_connect(&id) {
        return Err("unknown service".into());
    }
    signin::open_gemini(&app, false)
}

#[tauri::command]
async fn disconnect_service(app: AppHandle, id: String) -> Result<(), String> {
    if !signin::can_connect(&id) {
        return Err("unknown service".into());
    }
    signin::forget_gemini(&app).await?;
    refresh(app).await;
    Ok(())
}

#[tauri::command]
fn open_link(url: String) -> Result<(), String> {
    if !url.starts_with("https://") {
        return Err("only https links".into());
    }
    let mut command = if cfg!(target_os = "macos") {
        let mut c = std::process::Command::new("open");
        c.arg(&url);
        c
    } else if cfg!(windows) {
        let mut c = std::process::Command::new("rundll32");
        c.args(["url.dll,FileProtocolHandler", &url]);
        c
    } else {
        let mut c = std::process::Command::new("xdg-open");
        c.arg(&url);
        c
    };
    command.spawn().map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
fn quit(app: AppHandle) {
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(20))
        .user_agent(concat!("AImeter/", env!("CARGO_PKG_VERSION")))
        .build()
        .expect("http client");

    // `aimeter --icon-dump FILE PERCENT [colour]` writes the tray picture as raw
    // RGBA bytes, to look at it without starting the app.
    let args: Vec<String> = std::env::args().collect();
    if let Some(at) = args.iter().position(|arg| arg == "--icon-dump") {
        let percent = args.get(at + 2).and_then(|p| p.parse::<f64>().ok());
        let template = args.get(at + 3).map(String::as_str) != Some("colour");
        let picture = if args.get(at + 3).map(String::as_str) == Some("name") {
            icon::menu_bar_character()
        } else if args.get(at + 3).map(String::as_str) == Some("kvadrik") {
            icon::tray_character(percent)
        } else {
            icon::tray_image(percent, template)
        };
        if let Some(file) = args.get(at + 1) {
            let _ = std::fs::write(file, picture.rgba());
        }
        println!("{} {}", picture.width(), picture.height());
        return;
    }

    // `aimeter --json` prints what every service returned and exits.
    // Handy for checking a service without opening the panel.
    if std::env::args().any(|arg| arg == "--json") {
        let all = tauri::async_runtime::block_on(async {
            let mut all = Vec::new();
            for info in providers::ALL {
                all.push(providers::snapshot(info, &client).await);
            }
            all
        });
        println!("{}", serde_json::to_string_pretty(&all).unwrap_or_default());
        return;
    }

    let builder = tauri::Builder::default();
    // Windows: starting AI Meter while it already runs opens the running
    // one's panel instead of putting a second Kvadrik into the tray. This
    // plugin has to come before the others.
    #[cfg(windows)]
    let builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
        show_panel(app);
    }));
    builder
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .manage(Shared {
            client,
            snapshots: Mutex::new(Vec::new()),
            quiet_until: Mutex::new(HashMap::new()),
            busy: tokio::sync::Mutex::new(()),
            hidden_at: AtomicI64::new(0),
            show_left: AtomicBool::new(false),
            renewed_at: AtomicI64::new(0),
            tray_service: Mutex::new(None),
            auto_update: AtomicBool::new(true),
            hidden: Mutex::new(HashSet::new()),
            icon_percent: AtomicI64::new(-2),
        })
        .invoke_handler(tauri::generate_handler![
            get_snapshots,
            refresh_now,
            set_api_key,
            open_link,
            fit_panel,
            set_show_left,
            set_tray_service,
            set_hidden_services,
            set_auto_update,
            connect_service,
            disconnect_service,
            quit
        ])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Windows draws the same panel noticeably larger than macOS does;
            // shrink the page a little there (the UI scales its height to match).
            #[cfg(windows)]
            if let Some(panel) = app.get_webview_window("main") {
                let _ = panel.set_zoom(PANEL_ZOOM);
            }

            let refresh_item = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "Quit AI Meter", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&refresh_item, &quit_item])?;

            // macOS: text only ("AI 84%"). Elsewhere: the bubble picture.
            let tray = TrayIconBuilder::with_id("main");
            #[cfg(target_os = "macos")]
            let tray = tray.icon(icon::menu_bar_character()).icon_as_template(true);
            #[cfg(not(target_os = "macos"))]
            let tray = tray.icon(icon::tray_character(None));
            tray
                .tooltip("AI Meter")
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "refresh" => {
                        tauri::async_runtime::spawn(refresh(app.clone()));
                    }
                    "quit" => app.exit(0),
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    tauri_plugin_positioner::on_tray_event(tray.app_handle(), &event);
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        toggle_panel(tray.app_handle());
                    }
                })
                .build(app)?;

            tauri::async_runtime::spawn(updates::watch(app.handle().clone()));

            let handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    refresh(handle.clone()).await;
                    tokio::time::sleep(REFRESH_EVERY).await;
                }
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            // Only the panel hides when it loses focus; sign-in windows stay.
            if window.label() != "main" {
                return;
            }
            if let WindowEvent::Focused(false) = event {
                window
                    .app_handle()
                    .state::<Shared>()
                    .hidden_at
                    .store(providers::now_ms(), Ordering::Relaxed);
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running AI Meter");
}
