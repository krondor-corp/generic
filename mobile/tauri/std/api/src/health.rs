//! `GET /_status/livez` — is the server up. Public, no auth.

use reqwest::{Client as HttpClient, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::ApiRequest;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
}

/// A cheap reachability check. **Server
/// route to mirror:** `GET /_status/livez` (from `generic_rust_core::http::health`).
pub struct HealthRequest;

impl ApiRequest for HealthRequest {
    type Response = HealthResponse;
    fn build_request(self, base: &Url, http: &HttpClient) -> RequestBuilder {
        http.get(base.join("/_status/livez").expect("static path"))
    }
}
