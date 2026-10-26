mod sqlite;
pub mod types;

use std::ops::Deref;
use std::path::Path;

use sqlx::migrate::Migrator;
use sqlx::SqlitePool;

pub use types::DbUuid;

/// A connected, migrated SQLite pool. Derefs to [`SqlitePool`].
///
/// Migrations belong to the application: there is one `_sqlx_migrations`
/// table per database, so each app ships a single `sqlx::migrate!()`
/// directory containing both its own migrations and the Apalis ones it
/// needs. Pass that migrator here.
#[derive(Clone, Debug)]
pub struct Database(SqlitePool);

impl Database {
    /// Connect by URL (`sqlite://data/app.db`) and run `migrator`.
    pub async fn connect(
        database_url: &url::Url,
        migrator: &Migrator,
    ) -> Result<Self, DatabaseSetupError> {
        let pool = sqlite::connect(database_url).await?;
        sqlite::migrate(&pool, migrator).await?;
        Ok(Self(pool))
    }

    /// Connect to a database file by path and run `migrator`. Used where
    /// the location comes from the platform (Tauri's app data dir) rather
    /// than an env var.
    pub async fn connect_file(
        path: &Path,
        migrator: &Migrator,
    ) -> Result<Self, DatabaseSetupError> {
        let pool = sqlite::connect_file(path).await?;
        sqlite::migrate(&pool, migrator).await?;
        Ok(Self(pool))
    }
}

impl Deref for Database {
    type Target = SqlitePool;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DatabaseSetupError {
    #[error("migration failed: {0}")]
    MigrationFailed(sqlx::migrate::MigrateError),

    #[error("database unavailable: {0}")]
    Unavailable(sqlx::Error),
}
