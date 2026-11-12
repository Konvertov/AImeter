//! Quiet updates. A new version is downloaded in the background, checked
//! against our signing key by the updater itself, and put in place at a
//! moment when the panel is closed, so nothing changes under the user's hands.

use std::sync::atomic::Ordering;
use std::time::Duration;
use tauri::{AppHandle, Manager};
use tauri_plugin_updater::UpdaterExt;

const FIRST_CHECK_AFTER: Duration = Duration::from_secs(60);
const CHECK_EVERY: Duration = Duration::from_secs(4 * 3600);

pub async fn watch(app: AppHandle) {
    tokio::time::sleep(FIRST_CHECK_AFTER).await;
    loop {
        if app.state::<crate::Shared>().auto_update.load(Ordering::Relaxed) {
            // Any failure (no network, no release yet) just means "try again later".
            let _ = install_if_newer(&app).await;
        }
        tokio::time::sleep(CHECK_EVERY).await;
    }
}

async fn install_if_newer(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let Some(update) = app.updater()?.check().await? else {
        return Ok(());
    };
    let package = update.download(|_, _| {}, || {}).await?;

    // Wait until the panel is closed and no sign-in window is open.
    loop {
        let busy = app
            .webview_windows()
            .values()
            .any(|window| window.is_visible().unwrap_or(false));
        if !busy {
            break;
        }
        tokio::time::sleep(Duration::from_secs(20)).await;
    }
    if !app.state::<crate::Shared>().auto_update.load(Ordering::Relaxed) {
        return Ok(());
    }
    update.install(package)?;
    app.restart();
}
