//! `/api/v0/entries` — the rows the server writes and the device mirrors.

mod list;

use serde::{Deserialize, Serialize};

pub use list::{EntriesRequest, EntriesResponse, DEFAULT_LIMIT, MAX_LIMIT};

/// One row as the server stores it. `seq` is the server's monotonic
/// insert counter and is what devices page on — a row's `seq` never
/// changes, so "everything after the last `seq` I saw" is exactly "new
/// since last sync". `body` is whatever the server's job produced.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Entry {
    pub seq: i64,
    pub body: String,
    /// RFC 3339, when the server wrote it.
    pub created_at: String,
}
