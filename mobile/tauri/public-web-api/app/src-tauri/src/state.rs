use mobile_std::identity::Identity;
use mobile_std::settings::Settings;
use mobile_std::Database;
use tauri::{AppHandle, Runtime};
use tokio::sync::Mutex;

use crate::database::MIGRATOR;

/// Shared across commands and the poller via `app.state::<AppState>()`.
pub struct AppState {
    pub db: Database,
    pub settings: Settings,
    pub identity: Mutex<Identity>,
}

impl AppState {
    pub async fn init<R: Runtime>(app: &AppHandle<R>) -> mobile_std::Result<Self> {
        let db = mobile_std::db::open(app, &MIGRATOR).await?;
        let settings = Settings::new(db.clone());
        let identity = Identity::load_or_create(&settings).await?;
        tracing::info!(device_id = %identity.device_id, name = %identity.name, "app state ready");
        Ok(Self {
            db,
            settings,
            identity: Mutex::new(identity),
        })
    }
}

/// Settings keys this app adds on top of `mobile_std::settings::keys`.
pub mod keys {
    pub const CURSOR: &str = "sync.cursor";
    pub const LAST_SYNC_AT: &str = "sync.last_at";
    pub const LAST_ERROR: &str = "sync.last_error";
}
