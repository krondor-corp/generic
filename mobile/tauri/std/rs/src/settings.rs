//! A string key/value table for small, mutable app state: the server URL,
//! the API token, sync cursors, flags.
//!
//! Lives in its own table (`kv`) that [`crate::db::open`] creates
//! outside the app's migrations, so every app gets it without a
//! migration of its own. Anything with structure belongs in a real table.

use sqlx::Row;

use crate::{Database, Result};

pub async fn ensure_table(db: &Database) -> Result<()> {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS kv (
            key TEXT PRIMARY KEY NOT NULL,
            value TEXT NOT NULL,
            updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
        )",
    )
    .execute(&**db)
    .await?;
    Ok(())
}

#[derive(Clone, Debug)]
pub struct Settings {
    db: Database,
}

impl Settings {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let row = sqlx::query("SELECT value FROM kv WHERE key = ?")
            .bind(key)
            .fetch_optional(&*self.db)
            .await?;
        Ok(row.map(|r| r.get::<String, _>("value")))
    }

    pub async fn set(&self, key: &str, value: &str) -> Result<()> {
        sqlx::query(
            "INSERT INTO kv (key, value) VALUES (?, ?)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value,
                 updated_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')",
        )
        .bind(key)
        .bind(value)
        .execute(&*self.db)
        .await?;
        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<()> {
        sqlx::query("DELETE FROM kv WHERE key = ?")
            .bind(key)
            .execute(&*self.db)
            .await?;
        Ok(())
    }

    /// Typed convenience for integer cursors.
    pub async fn get_i64(&self, key: &str) -> Result<Option<i64>> {
        Ok(self.get(key).await?.and_then(|v| v.parse().ok()))
    }

    pub async fn set_i64(&self, key: &str, value: i64) -> Result<()> {
        self.set(key, &value.to_string()).await
    }
}

/// Keys the std lib itself reads. Apps add their own alongside.
pub mod keys {
    pub const SERVER_URL: &str = "server.url";
    pub const SERVER_TOKEN: &str = "server.token";
    pub const DEVICE_ID: &str = "device.id";
    pub const DEVICE_NAME: &str = "device.name";
}
