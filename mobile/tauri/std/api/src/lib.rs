//! HTTP contract between the mobile app and `public-web-api/server`.
//!
//! ## The pattern
//!
//! Every endpoint's **request type** implements [`ApiRequest`]: it owns
//! its own HTTP method, path, query, and body by building a
//! [`reqwest::RequestBuilder`]. The [`Client`] is generic over the trait
//! — `client.call(SomeRequest { .. }).await?` sends it and decodes the
//! typed [`ApiRequest::Response`].
//!
//! ```ignore
//! impl ApiRequest for StoriesRequest {
//!     type Response = StoriesResponse;
//!     fn build_request(self, base: &Url, http: &HttpClient) -> RequestBuilder {
//!         http.get(base.join("/api/v0/stories").unwrap()).query(&self)
//!     }
//! }
//! ```
//!
//! Auth is a property of the *client*, not the request:
//! [`Client::with_bearer`] attaches `Authorization: Bearer <token>` to
//! every call, so request types stay auth-agnostic.
//!
//! ## Modules
//!
//! One module per resource, one file per endpoint inside it. Wire types
//! shared by several endpoints sit in the resource's `mod.rs`.
//!
//! - [`stories`] — `GET /api/v0/stories` (cursor-paged feed).
//! - [`devices`] — `POST /api/v0/devices` (register this install).
//! - [`events`] — the SSE event payloads on `GET /api/v0/events`.
//! - [`health`] — `GET /_status/livez`.

use reqwest::{Client as HttpClient, RequestBuilder};
use serde::de::DeserializeOwned;

mod error;

pub mod devices;
pub mod events;
pub mod health;
pub mod stories;

pub use error::ApiError;

/// Re-exported so callers can build a base [`Url`] without adding their
/// own `url` dependency.
pub use reqwest::Url;

/// A typed HTTP request. The implementor owns everything about how the
/// request is formed — method, path, query, body — by returning a
/// fully-built [`RequestBuilder`]. The [`Client`] only sends it and
/// decodes [`Self::Response`].
pub trait ApiRequest {
    type Response: DeserializeOwned;
    fn build_request(self, base_url: &Url, client: &HttpClient) -> RequestBuilder;
}

/// Generic typed client: holds a base URL + a reqwest client and
/// executes any [`ApiRequest`].
#[derive(Debug, Clone)]
pub struct Client {
    base: Url,
    http: HttpClient,
}

impl Client {
    pub fn new(base: &Url) -> Result<Self, ApiError> {
        Ok(Self {
            base: base.clone(),
            http: HttpClient::builder().build()?,
        })
    }

    /// Build over an existing reqwest client (lets callers share a
    /// connection pool / configured client).
    pub fn with_http(base: &Url, http: HttpClient) -> Self {
        Self {
            base: base.clone(),
            http,
        }
    }

    /// Build a client that attaches `Authorization: Bearer <token>` to
    /// every request.
    pub fn with_bearer(base: &Url, token: &str) -> Result<Self, ApiError> {
        use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
        let mut headers = HeaderMap::new();
        let mut value = HeaderValue::from_str(&format!("Bearer {token}"))
            .map_err(|e| ApiError::Other(e.to_string()))?;
        value.set_sensitive(true);
        headers.insert(AUTHORIZATION, value);
        let http = HttpClient::builder().default_headers(headers).build()?;
        Ok(Self {
            base: base.clone(),
            http,
        })
    }

    pub fn base(&self) -> &Url {
        &self.base
    }

    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Send `req` (method/path/body per its own `build_request`) and
    /// decode the typed reply. Non-2xx → [`ApiError::HttpStatus`]. A
    /// success with an empty body (204-style action endpoints) decodes
    /// as JSON `null`, so `Response = ()` works.
    pub async fn call<R: ApiRequest>(&self, req: R) -> Result<R::Response, ApiError> {
        let response = req.build_request(&self.base, &self.http).send().await?;
        let status = response.status();
        if status.is_success() {
            let body = response.bytes().await?;
            if body.is_empty() {
                Ok(serde_json::from_slice(b"null")?)
            } else {
                Ok(serde_json::from_slice(&body)?)
            }
        } else {
            Err(ApiError::HttpStatus(
                status,
                response.text().await.unwrap_or_default(),
            ))
        }
    }
}
