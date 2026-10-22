use mobile_api::stories::Story as WireStory;

use crate::database::Database;

/// A story as mirrored on the device. Same shape as the wire type plus
/// when we synced it; converts back to the wire type for the frontend so
/// there is one `Story` interface in TypeScript.
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct Story {
    seq: i64,
    id: i64,
    title: String,
    url: Option<String>,
    author: String,
    score: i64,
    posted_at: i64,
    fetched_at: String,
    synced_at: String,
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

    pub fn synced_at(&self) -> &str {
        &self.synced_at
    }

    pub fn to_wire(&self) -> WireStory {
        WireStory {
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

    /// Insert if unseen. Returns whether a row was added — the signal for
    /// "is this new to the user", which is what notifications key on.
    pub async fn insert_if_new(story: &WireStory, db: &Database) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            r#"
            INSERT OR IGNORE INTO stories (seq, id, title, url, author, score, posted_at, fetched_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(story.seq)
        .bind(story.id)
        .bind(&story.title)
        .bind(&story.url)
        .bind(&story.by)
        .bind(story.score)
        .bind(story.posted_at)
        .bind(&story.fetched_at)
        .execute(&**db)
        .await?;
        Ok(result.rows_affected() == 1)
    }

    /// Newest first.
    pub async fn list(limit: u32, db: &Database) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as::<_, Story>("SELECT * FROM stories ORDER BY seq DESC LIMIT ?")
            .bind(limit)
            .fetch_all(&**db)
            .await
    }

    pub async fn count(db: &Database) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM stories")
            .fetch_one(&**db)
            .await
    }

    pub async fn delete_all(db: &Database) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM stories").execute(&**db).await?;
        Ok(())
    }
}
