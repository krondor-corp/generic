//! `GET /api/v0/stories?since=<seq>&limit=<n>` — stories newer than a cursor.

use reqwest::{Client as HttpClient, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::ApiRequest;

use super::Story;

pub const DEFAULT_LIMIT: u32 = 50;
pub const MAX_LIMIT: u32 = 200;

/// Stories with `seq > since`, oldest first, at most `limit`. Omit
/// `since` for the initial backfill (the server returns the most recent
/// `limit` in that case). **Server route to mirror:**
/// `GET /api/v0/stories` (RequireBearer).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoriesRequest {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub since: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoriesResponse {
    pub stories: Vec<Story>,
    /// Highest `seq` on the server right now — pass back as `since`.
    /// Equal to the request's `since` when nothing is new.
    pub cursor: i64,
}

impl ApiRequest for StoriesRequest {
    type Response = StoriesResponse;
    fn build_request(self, base: &Url, http: &HttpClient) -> RequestBuilder {
        http.get(base.join("/api/v0/stories").expect("static path"))
            .query(&self)
    }
}
