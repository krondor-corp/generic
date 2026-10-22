use serde::Serialize;

use crate::database::Database;

/// A row as stored. Private fields with getters; [`Entry::to_wire`]
/// produces the `mobile-api` type devices receive.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Entry {
    seq: i64,
    body: String,
    created_at: String,
}

impl Entry {
    pub fn seq(&self) -> i64 {
        self.seq
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn created_at(&self) -> &str {
        &self.created_at
    }

    pub fn to_wire(&self) -> mobile_api::entries::Entry {
        mobile_api::entries::Entry {
            seq: self.seq,
            body: self.body.clone(),
            created_at: self.created_at.clone(),
        }
    }

    pub async fn create(body: &str, db: &Database) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Entry>("INSERT INTO entries (body) VALUES (?) RETURNING *")
            .bind(body)
            .fetch_one(&**db)
            .await
    }

    /// Entries with `seq > since`, oldest first. With no cursor, the most
    /// recent `limit` (still returned oldest first) so a fresh device gets
    /// a sensible backfill rather than the beginning of time.
    pub async fn list_since(
        since: Option<i64>,
        limit: u32,
        db: &Database,
    ) -> Result<Vec<Self>, sqlx::Error> {
        match since {
            Some(since) => {
                sqlx::query_as::<_, Entry>(
                    "SELECT * FROM entries WHERE seq > ? ORDER BY seq ASC LIMIT ?",
                )
                .bind(since)
                .bind(limit)
                .fetch_all(&**db)
                .await
            }
            None => sqlx::query_as::<_, Entry>(
                "SELECT * FROM (SELECT * FROM entries ORDER BY seq DESC LIMIT ?) ORDER BY seq ASC",
            )
            .bind(limit)
            .fetch_all(&**db)
            .await,
        }
    }

    /// The most recent `limit` entries, newest first (for the web page).
    pub async fn latest(limit: u32, db: &Database) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Entry>("SELECT * FROM entries ORDER BY seq DESC LIMIT ?")
            .bind(limit)
            .fetch_all(&**db)
            .await
    }

    /// Current cursor: the highest `seq`, or 0 when empty.
    pub async fn max_seq(db: &Database) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COALESCE(MAX(seq), 0) FROM entries")
            .fetch_one(&**db)
            .await
    }

    pub async fn count(db: &Database) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM entries")
            .fetch_one(&**db)
            .await
    }

    /// Drop everything but the newest `keep` rows. Returns how many went.
    /// Devices whose cursor is older than the oldest surviving row simply
    /// resume from the next row that exists; `seq` never reuses a number.
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
