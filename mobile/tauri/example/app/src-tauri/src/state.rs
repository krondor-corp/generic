use mobile_std::api::ServerConfig;
use mobile_std::settings::Settings;
use mobile_std::Database;
use tauri::{AppHandle, Runtime};

use crate::database::MIGRATOR;

/// Where the app points. The server is a free public API, so there is
/// no setup screen: debug builds (`make dev`, `make dev-ios`,
/// `make dev-android`) talk to the local dev server, release builds to
/// the deployed `ios` service.
pub const DEFAULT_SERVER_URL: &str = if cfg!(debug_assertions) {
    DEV_SERVER_URL
} else {
    "https://ios.generic.krondor.org"
};

/// The Android emulator reaches the host machine at 10.0.2.2; desktop and
/// the iOS simulator at localhost.
#[cfg(target_os = "android")]
const DEV_SERVER_URL: &str = "http://10.0.2.2:9100";
#[cfg(not(target_os = "android"))]
const DEV_SERVER_URL: &str = "http://localhost:9100";

/// Shared across commands and the poller via `app.state::<AppState>()`.
pub struct AppState {
    pub db: Database,
    pub settings: Settings,
}

impl AppState {
    pub async fn init<R: Runtime>(app: &AppHandle<R>) -> mobile_std::Result<Self> {
        let db = mobile_std::db::open(app, &MIGRATOR).await?;
        let settings = Settings::new(db.clone());
        ServerConfig::seed_default(&settings, DEFAULT_SERVER_URL).await?;
        tracing::info!("app state ready");
        Ok(Self { db, settings })
    }
}

/// Settings keys this app adds on top of `mobile_std::settings::keys`.
pub mod keys {
    pub const CURSOR: &str = "sync.cursor";
    pub const LAST_SYNC_AT: &str = "sync.last_at";
    pub const LAST_ERROR: &str = "sync.last_error";
}
