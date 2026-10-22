use mobile_api::stories::Story as WireStory;
use tauri::{AppHandle, State};
use tauri_plugin_opener::OpenerExt;

use crate::database::models::Story;
use crate::state::AppState;
use crate::sync::{self, SyncReport};

#[tauri::command]
pub async fn list_stories(
    limit: Option<u32>,
    state: State<'_, AppState>,
) -> mobile_std::Result<Vec<WireStory>> {
    let stories = Story::list(limit.unwrap_or(100).clamp(1, 500), &state.db).await?;
    Ok(stories.iter().map(Story::to_wire).collect())
}

#[tauri::command]
pub async fn sync_now(app: AppHandle) -> mobile_std::Result<SyncReport> {
    sync::run(app).await
}

/// Open in the system browser. Done here rather than via the opener
/// plugin's JS API so the webview needs no extra npm dependency and the
/// capability stays Rust-side.
#[tauri::command]
pub async fn open_story(url: String, app: AppHandle) -> mobile_std::Result<()> {
    app.opener()
        .open_url(url, None::<&str>)
        .map_err(|e| mobile_std::Error::Other(e.to_string()))
}
