use mobile_api::entries::Entry as WireEntry;

use crate::database::Database;

/// An entry as mirrored on the device. Same shape as the wire type plus
/// when we synced it; converts back to the wire type for the frontend so
/// there is one `Entry` interface in TypeScript.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Entry {
    seq: i64,
    body: String,
    created_at: String,
    synced_at: String,
}

impl Entry {
    pub fn seq(&self) -> i64 {
        self.seq
    }

    pub fn synced_at(&self) -> &str {
        &self.synced_at
    }

    pub fn to_wire(&self) -> WireEntry {
        WireEntry {
            seq: self.seq,
            body: self.body.clone(),
            created_at: self.created_at.clone(),
        }
    }

    /// Insert if unseen. Returns whether a row was added — the signal for
    /// "is this new to the user".
    pub async fn insert_if_new(entry: &WireEntry, db: &Database) -> Result<bool, sqlx::Error> {
        let result =
            sqlx::query("INSERT OR IGNORE INTO entries (seq, body, created_at) VALUES (?, ?, ?)")
                .bind(entry.seq)
                .bind(&entry.body)
                .bind(&entry.created_at)
                .execute(&**db)
                .await?;
        Ok(result.rows_affected() == 1)
    }

    /// Newest first.
    pub async fn list(limit: u32, db: &Database) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Entry>("SELECT * FROM entries ORDER BY seq DESC LIMIT ?")
            .bind(limit)
            .fetch_all(&**db)
            .await
    }

    pub async fn count(db: &Database) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM entries")
            .fetch_one(&**db)
            .await
    }

    /// Keep the mirror bounded too: drop everything but the newest `keep`.
    pub async fn prune(keep: u32, db: &Database) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM entries WHERE seq <= (SELECT COALESCE(MAX(seq), 0) FROM entries) - ?",
        )
        .bind(keep)
        .execute(&**db)
        .await?;
        Ok(result.rows_affected())
    }
}
