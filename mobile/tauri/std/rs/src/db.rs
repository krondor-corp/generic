//! SQLite on the device.
//!
//! The file lives in Tauri's per-app data directory (`~/Library/Application
//! Support/<bundle id>` on macOS, the sandboxed container on iOS, the
//! app's files dir on Android). The app owns its migrations exactly like a
//! server does — pass its `sqlx::migrate!()`.

use std::path::PathBuf;

use sqlx::migrate::Migrator;
use tauri::{AppHandle, Manager, Runtime};

use crate::{Database, Result};

pub const DB_FILE: &str = "app.db";

/// Where the database file lives for this app.
pub fn path<R: Runtime>(app: &AppHandle<R>) -> Result<PathBuf> {
    Ok(app.path().app_data_dir()?.join(DB_FILE))
}

/// Open (creating the directory and file if needed) and migrate.
pub async fn open<R: Runtime>(app: &AppHandle<R>, migrator: &Migrator) -> Result<Database> {
    let path = path(app)?;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    tracing::info!(path = %path.display(), "opening database");
    let db = Database::connect_file(&path, migrator).await?;
    crate::settings::ensure_table(&db).await?;
    Ok(db)
}
