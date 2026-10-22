use mobile_api::health::HealthRequest;
use mobile_api::{Client, Url};
use mobile_std::api::ServerConfig;
use mobile_std::poller::Poller;
use tauri::State;

use crate::commands::status::{self, Status};
use crate::database::models::Story;
use crate::state::{keys, AppState};

#[tauri::command]
pub async fn save_settings(
    url: String,
    token: String,
    device_name: String,
    state: State<'_, AppState>,
    poller: State<'_, Poller>,
) -> mobile_std::Result<Status> {
    ServerConfig {
        url: url.trim().trim_end_matches('/').to_string(),
        token: token.trim().to_string(),
    }
    .save(&state.settings)
    .await?;

    let name = device_name.trim();
    if !name.is_empty() {
        state
            .identity
            .lock()
            .await
            .rename(&state.settings, name)
            .await?;
    }

    // New server, new news: don't make the user wait for the next tick.
    poller.trigger();
    status::build(&state).await
}

/// Hit `/_status/livez` with the *unsaved* form values so the user can
/// check before committing.
#[tauri::command]
pub async fn test_connection(url: String, token: String) -> mobile_std::Result<String> {
    let base = Url::parse(url.trim())?;
    let client = Client::with_bearer(&base, token.trim())?;
    let health = client.call(HealthRequest).await?;
    Ok(format!("Server reachable: {}", health.status))
}

/// Drop mirrored stories and the cursor; keeps server settings and
/// identity. Next sync backfills.
#[tauri::command]
pub async fn reset_data(state: State<'_, AppState>) -> mobile_std::Result<Status> {
    Story::delete_all(&state.db).await?;
    state.settings.delete(keys::CURSOR).await?;
    state.settings.delete(keys::LAST_SYNC_AT).await?;
    state.settings.delete(keys::LAST_ERROR).await?;
    status::build(&state).await
}
