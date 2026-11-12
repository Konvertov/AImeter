//! Sign-in windows for services that only show limits to a signed-in
//! browser session. The user signs in on the service's own site inside a
//! window of AImeter; AImeter never sees the password, only the session
//! the site hands out afterwards.

use crate::providers::{self, gemini};
use crate::secrets;
use std::time::Duration;
use tauri::webview::PageLoadEvent;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

const WINDOW: &str = "signin-gemini";

/// Open the Gemini sign-in window. `quiet` opens it invisibly, to renew a
/// session the built-in browser still remembers.
pub fn open_gemini(app: &AppHandle, quiet: bool) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window(WINDOW) {
        if !quiet {
            let _ = existing.show();
            let _ = existing.set_focus();
        }
        return Ok(());
    }
    let address = format!("{}/usage", gemini::SITE).parse().map_err(|_| "bad address".to_string())?;
    let handle = app.clone();
    let window = WebviewWindowBuilder::new(app, WINDOW, WebviewUrl::External(address))
        .title("Gemini")
        .inner_size(520.0, 720.0)
        .visible(!quiet)
        .on_page_load(move |window, payload| {
            if payload.event() == PageLoadEvent::Finished
                && payload.url().host_str() == Some("gemini.google.com")
            {
                tauri::async_runtime::spawn(capture(handle.clone(), window, quiet));
            }
        })
        .build()
        .map_err(|e| e.to_string())?;
    if quiet {
        // Never leave an invisible window behind.
        tauri::async_runtime::spawn(async move {
            tokio::time::sleep(Duration::from_secs(40)).await;
            let _ = window.close();
        });
    }
    Ok(())
}

/// After a Gemini page loaded: if the site now answers with usage numbers,
/// keep the session and close the window.
async fn capture(app: AppHandle, window: WebviewWindow, quiet: bool) {
    tokio::time::sleep(Duration::from_millis(1500)).await;
    let Ok(address) = gemini::SITE.parse() else { return };
    let Ok(cookies) = window.cookies_for_url(address) else { return };
    if cookies.is_empty() {
        return;
    }
    let session = cookies
        .iter()
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .collect::<Vec<_>>()
        .join("; ");
    let client = app.state::<crate::Shared>().client.clone();
    if gemini::ask(&client, &session).await.is_err() {
        // Not signed in yet: the visible window stays open for the user.
        if quiet {
            let _ = window.close();
        }
        return;
    }
    let saved = tauri::async_runtime::spawn_blocking(move || secrets::save_api_key("gemini", &session)).await;
    if matches!(saved, Ok(Ok(()))) {
        let _ = window.close();
        crate::refresh(app).await;
    }
}

/// Forget the Gemini session: both the saved copy and the built-in browser's.
pub async fn forget_gemini(app: &AppHandle) -> Result<(), String> {
    tauri::async_runtime::spawn_blocking(|| secrets::save_api_key("gemini", ""))
        .await
        .map_err(|e| e.to_string())??;
    if let Some(open) = app.get_webview_window(WINDOW) {
        let _ = open.clear_all_browsing_data();
        let _ = open.close();
    } else if let Ok(blank) = "about:blank".parse() {
        if let Ok(window) = WebviewWindowBuilder::new(app, WINDOW, WebviewUrl::External(blank))
            .visible(false)
            .build()
        {
            let _ = window.clear_all_browsing_data();
            let _ = window.close();
        }
    }
    Ok(())
}

pub fn can_connect(id: &str) -> bool {
    providers::info(id).is_some_and(|info| info.connect)
}
