//! `GET /api/v0/entries?since=<seq>&limit=<n>` — entries newer than a cursor.

use reqwest::{Client as HttpClient, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::ApiRequest;

use super::Entry;

pub const DEFAULT_LIMIT: u32 = 50;
pub const MAX_LIMIT: u32 = 200;

/// Entries with `seq > since`, oldest first, at most `limit`. Omit
/// `since` for the initial backfill (the server returns the most recent
/// `limit` in that case). **Server route to mirror:**
/// `GET /api/v0/entries`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EntriesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntriesResponse {
    pub entries: Vec<Entry>,
    /// Highest `seq` on the server right now — pass back as `since`.
    /// Equal to the request's `since` when nothing is new.
    pub cursor: i64,
}

impl ApiRequest for EntriesRequest {
    type Response = EntriesResponse;
    fn build_request(self, base: &Url, http: &HttpClient) -> RequestBuilder {
        http.get(base.join("/api/v0/entries").expect("static path"))
            .query(&self)
    }
}
