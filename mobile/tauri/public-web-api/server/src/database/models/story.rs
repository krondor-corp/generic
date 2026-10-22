use serde::Serialize;

use crate::database::Database;

/// A story as stored. Private fields with getters; [`Story::to_wire`]
/// produces the `mobile-api` type devices receive.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Story {
    seq: i64,
    id: i64,
    title: String,
    url: Option<String>,
    author: String,
    score: i64,
    posted_at: i64,
    fetched_at: String,
    updated_at: String,
}

/// What the scraper hands us for a story it has not seen before.
#[derive(Debug, Clone)]
pub struct NewStory {
    pub id: i64,
    pub title: String,
    pub url: Option<String>,
    pub author: String,
    pub score: i64,
    pub posted_at: i64,
}

impl Story {
    pub fn seq(&self) -> i64 {
        self.seq
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn author(&self) -> &str {
        &self.author
    }

    pub fn score(&self) -> i64 {
        self.score
    }

    pub fn posted_at(&self) -> i64 {
        self.posted_at
    }

    pub fn fetched_at(&self) -> &str {
        &self.fetched_at
    }

    pub fn to_wire(&self) -> mobile_api::stories::Story {
        mobile_api::stories::Story {
            id: self.id,
            seq: self.seq,
            title: self.title.clone(),
            url: self.url.clone(),
            by: self.author.clone(),
            score: self.score,
            posted_at: self.posted_at,
            fetched_at: self.fetched_at.clone(),
        }
    }

    pub async fn create(new: NewStory, db: &Database) -> Result<Self, sqlx::Error> {
        sqlx::query_as::<_, Story>(
            r#"
            INSERT INTO stories (id, title, url, author, score, posted_at)
            VALUES (?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(new.id)
        .bind(&new.title)
        .bind(&new.url)
        .bind(&new.author)
        .bind(new.score)
        .bind(new.posted_at)
        .fetch_one(&**db)
        .await
    }

    pub async fn find_by_id(id: i64, db: &Database) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as::<_, Story>("SELECT * FROM stories WHERE id = ?")
            .bind(id)
            .fetch_optional(&**db)
            .await
    }

    /// Which of `ids` we already have.
    pub async fn known_ids(ids: &[i64], db: &Database) -> Result<Vec<i64>, sqlx::Error> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }
        let placeholders = vec!["?"; ids.len()].join(", ");
        let sql = format!("SELECT id FROM stories WHERE id IN ({placeholders})");
        let mut query = sqlx::query_scalar::<_, i64>(&sql);
        for id in ids {
            query = query.bind(id);
        }
        query.fetch_all(&**db).await
    }

    /// Stories with `seq > since`, oldest first. With no cursor, the most
    /// recent `limit` (still returned oldest first) so a fresh device gets
    /// a sensible backfill rather than the beginning of time.
    pub async fn list_since(
        since: Option<i64>,
        limit: u32,
        db: &Database,
    ) -> Result<Vec<Self>, sqlx::Error> {
        match since {
            Some(since) => {
                sqlx::query_as::<_, Story>(
                    "SELECT * FROM stories WHERE seq > ? ORDER BY seq ASC LIMIT ?",
                )
                .bind(since)
                .bind(limit)
                .fetch_all(&**db)
                .await
            }
            None => sqlx::query_as::<_, Story>(
                "SELECT * FROM (SELECT * FROM stories ORDER BY seq DESC LIMIT ?) ORDER BY seq ASC",
            )
            .bind(limit)
            .fetch_all(&**db)
            .await,
        }
    }

    /// Current cursor: the highest `seq`, or 0 when empty.
    pub async fn max_seq(db: &Database) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COALESCE(MAX(seq), 0) FROM stories")
            .fetch_one(&**db)
            .await
    }

    pub async fn count(db: &Database) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM stories")
            .fetch_one(&**db)
            .await
    }
}
