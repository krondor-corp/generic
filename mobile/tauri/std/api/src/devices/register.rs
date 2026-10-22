//! `POST /api/v0/devices` — register (or touch) this install.

use reqwest::{Client as HttpClient, RequestBuilder, Url};
use serde::{Deserialize, Serialize};

use crate::ApiRequest;

use super::{Device, Platform};

/// Idempotent on `device_id`: a repeat call updates `name`/`platform` and
/// bumps `last_seen_at`. Call it on every launch and after every
/// successful sync. **Server route to mirror:** `POST /api/v0/devices`
/// (RequireBearer).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterDeviceRequest {
    pub device_id: String,
    pub platform: Platform,
    pub name: String,
}

impl ApiRequest for RegisterDeviceRequest {
    type Response = Device;
    fn build_request(self, base: &Url, http: &HttpClient) -> RequestBuilder {
        http.post(base.join("/api/v0/devices").expect("static path"))
            .json(&self)
    }
}
