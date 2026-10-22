//! `/api/v0/stories` — the feed the server scrapes and the device mirrors.

mod list;

use serde::{Deserialize, Serialize};

pub use list::{StoriesRequest, StoriesResponse, DEFAULT_LIMIT, MAX_LIMIT};

/// One story as the server stores it. `id` is the upstream (Hacker
/// News) item id; `seq` is the server's monotonic insert counter and is
/// what devices page on — a story's `seq` never changes, so "everything
/// after the last `seq` I saw" is exactly "new since last sync".
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Story {
    pub id: i64,
    pub seq: i64,
    pub title: String,
    pub url: Option<String>,
    pub by: String,
    pub score: i64,
    /// Unix seconds, from upstream.
    pub posted_at: i64,
    /// RFC 3339, when the server first saw it.
    pub fetched_at: String,
}
