use mobile_std::api::ServerConfig;
use serde::Serialize;
use tauri::State;

use crate::database::models::Entry;
use crate::state::{keys, AppState};

/// Everything the status line shows.
#[derive(Debug, Serialize)]
pub struct Status {
    pub server_url: Option<String>,
    pub cursor: Option<i64>,
    pub entry_count: i64,
    pub last_sync_at: Option<String>,
    pub last_error: Option<String>,
}

#[tauri::command]
pub async fn get_status(state: State<'_, AppState>) -> mobile_std::Result<Status> {
    Ok(Status {
        server_url: ServerConfig::load(&state.settings).await?.map(|c| c.url),
        cursor: state.settings.get_i64(keys::CURSOR).await?,
        entry_count: Entry::count(&state.db).await?,
        last_sync_at: state.settings.get(keys::LAST_SYNC_AT).await?,
        last_error: state.settings.get(keys::LAST_ERROR).await?,
    })
}
