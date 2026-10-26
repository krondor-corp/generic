use mobile_api::entries::Entry as WireEntry;
use tauri::{AppHandle, State};

use crate::database::models::Entry;
use crate::state::AppState;
use crate::sync::{self, SyncReport};

#[tauri::command]
pub async fn list_entries(
    limit: Option<u32>,
    state: State<'_, AppState>,
) -> mobile_std::Result<Vec<WireEntry>> {
    let entries = Entry::list(limit.unwrap_or(100).clamp(1, 500), &state.db).await?;
    Ok(entries.iter().map(Entry::to_wire).collect())
}

#[tauri::command]
pub async fn sync_now(app: AppHandle) -> mobile_std::Result<SyncReport> {
    sync::run(app).await
}
