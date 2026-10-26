//! Local notifications through `tauri-plugin-notification`.
//!
//! "Local" means scheduled by the app on the device — no APNs/FCM, no
//! server push. They fire while the app is running (foreground on
//! mobile, any state on desktop). That is enough for "you synced and
//! found something new"; waking a backgrounded phone is a different,
//! OS-scheduled problem not covered here.
//!
//! The app must register the plugin (`tauri_plugin_notification::init()`)
//! and grant `notification:default` in its capabilities file.

use tauri::{AppHandle, Runtime};
use tauri_plugin_notification::{NotificationExt, PermissionState};

use crate::Result;

/// Ask once if undetermined. Returns whether we may notify.
pub fn ensure_permission<R: Runtime>(app: &AppHandle<R>) -> Result<bool> {
    let state = match app.notification().permission_state()? {
        PermissionState::Prompt | PermissionState::PromptWithRationale => {
            app.notification().request_permission()?
        }
        other => other,
    };
    Ok(matches!(state, PermissionState::Granted))
}

/// Show a notification now. Silently no-ops if permission is denied so
/// callers don't have to branch — the sync still happened.
pub fn notify<R: Runtime>(app: &AppHandle<R>, title: &str, body: &str) -> Result<()> {
    if !ensure_permission(app)? {
        tracing::debug!("notification permission not granted; skipping");
        return Ok(());
    }
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()?;
    Ok(())
}
