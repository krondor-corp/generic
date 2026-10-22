use mobile_api::devices::Platform;
use mobile_std::api::ServerConfig;
use serde::Serialize;
use tauri::State;

use crate::database::models::Story;
use crate::state::{keys, AppState};

#[derive(Debug, Serialize)]
pub struct DeviceInfo {
    pub device_id: String,
    pub name: String,
    pub platform: Platform,
}

/// Everything the settings screen and status bar show.
#[derive(Debug, Serialize)]
pub struct Status {
    pub server: Option<ServerConfig>,
    pub device: DeviceInfo,
    pub cursor: Option<i64>,
    pub story_count: i64,
    pub last_sync_at: Option<String>,
    pub last_error: Option<String>,
}

pub async fn build(state: &AppState) -> mobile_std::Result<Status> {
    let identity = state.identity.lock().await;
    Ok(Status {
        server: ServerConfig::load(&state.settings).await?,
        device: DeviceInfo {
            device_id: identity.device_id.clone(),
            name: identity.name.clone(),
            platform: identity.platform,
        },
        cursor: state.settings.get_i64(keys::CURSOR).await?,
        story_count: Story::count(&state.db).await?,
        last_sync_at: state.settings.get(keys::LAST_SYNC_AT).await?,
        last_error: state.settings.get(keys::LAST_ERROR).await?,
    })
}

#[tauri::command]
pub async fn get_status(state: State<'_, AppState>) -> mobile_std::Result<Status> {
    build(&state).await
}
