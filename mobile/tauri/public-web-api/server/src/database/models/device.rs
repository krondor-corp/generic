use mobile_api::devices::Platform;
use serde::Serialize;
use uuid::Uuid;

use crate::database::types::DbUuid;
use crate::database::Database;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Device {
    id: DbUuid,
    device_id: String,
    platform: String,
    name: String,
    created_at: String,
    last_seen_at: String,
}

impl Device {
    pub fn id(&self) -> Uuid {
        self.id.into()
    }

    pub fn device_id(&self) -> &str {
        &self.device_id
    }

    pub fn platform(&self) -> Platform {
        self.platform.parse().unwrap_or(Platform::Unknown)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn last_seen_at(&self) -> &str {
        &self.last_seen_at
    }

    pub fn to_wire(&self) -> mobile_api::devices::Device {
        mobile_api::devices::Device {
            id: self.id(),
            device_id: self.device_id.clone(),
            platform: self.platform(),
            name: self.name.clone(),
            last_seen_at: self.last_seen_at.clone(),
        }
    }

    /// Register or touch. Keyed on the device-minted `device_id`.
    pub async fn upsert(
        device_id: &str,
        platform: Platform,
        name: &str,
        db: &Database,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Device>(
            r#"
            INSERT INTO devices (id, device_id, platform, name)
            VALUES (?, ?, ?, ?)
            ON CONFLICT(device_id) DO UPDATE SET
                platform = excluded.platform,
                name = excluded.name,
                last_seen_at = strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
            RETURNING *
            "#,
        )
        .bind(DbUuid::new())
        .bind(device_id)
        .bind(platform.as_str())
        .bind(name)
        .fetch_one(&**db)
        .await
    }

    pub async fn list(db: &Database) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Device>("SELECT * FROM devices ORDER BY last_seen_at DESC")
            .fetch_all(&**db)
            .await
    }

    pub async fn count(db: &Database) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM devices")
            .fetch_one(&**db)
            .await
    }
}
