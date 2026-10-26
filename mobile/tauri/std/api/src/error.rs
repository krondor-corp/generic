//! Client-side API error.

use reqwest::StatusCode;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("HTTP request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("URL parse: {0}")]
    UrlParse(#[from] url::ParseError),
    #[error("HTTP {0}: {1}")]
    HttpStatus(StatusCode, String),
    #[error("decode response: {0}")]
    Decode(#[from] serde_json::Error),
    #[error("{0}")]
    Other(String),
}

impl ApiError {
    /// `401`/`403` — the bearer token is missing or wrong. Callers show
    /// a "check your settings" state instead of retrying.
    pub fn is_unauthorized(&self) -> bool {
        matches!(
            self,
            ApiError::HttpStatus(StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN, _)
        )
    }
}
