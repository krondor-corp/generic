//! Hacker News' public read-only JSON API.
//!
//! Two endpoints: a list of ids and an item by id. No auth, no keys, no
//! rate-limit headers. It happens to be hosted on Firebase; that is
//! invisible here — it is just HTTPS GET returning JSON.
//!
//!   GET {base}/topstories.json  → [45231987, 45231654, ...]
//!   GET {base}/item/{id}.json   → {"id":..,"type":"story","title":..,"url":..,"by":..,"score":..,"time":..}

use serde::Deserialize;
use url::Url;

#[derive(Debug, Clone)]
pub struct HackerNews {
    http: reqwest::Client,
    base: Url,
}

#[derive(Debug, thiserror::Error)]
pub enum UpstreamError {
    #[error("request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("bad url: {0}")]
    Url(#[from] url::ParseError),
}

/// The subset of an HN item we keep. Every field except `id` is optional
/// upstream (deleted/dead items are nearly empty).
#[derive(Debug, Clone, Deserialize)]
pub struct Item {
    pub id: i64,
    #[serde(rename = "type")]
    pub kind: Option<String>,
    pub title: Option<String>,
    pub url: Option<String>,
    pub by: Option<String>,
    pub score: Option<i64>,
    /// Unix seconds.
    pub time: Option<i64>,
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub dead: bool,
}

impl Item {
    /// Worth storing: a live story with a title.
    pub fn is_story(&self) -> bool {
        self.kind.as_deref() == Some("story") && !self.deleted && !self.dead && self.title.is_some()
    }
}

impl HackerNews {
    pub fn new(base: Url) -> Result<Self, reqwest::Error> {
        let http = reqwest::Client::builder()
            .user_agent(concat!("public-web-api/", env!("CARGO_PKG_VERSION")))
            .timeout(std::time::Duration::from_secs(15))
            .build()?;
        Ok(Self { http, base })
    }

    pub async fn top_stories(&self) -> Result<Vec<i64>, UpstreamError> {
        let url = self.base.join("topstories.json")?;
        Ok(self
            .http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    /// `None` when the item does not exist (HN returns literal `null`).
    pub async fn item(&self, id: i64) -> Result<Option<Item>, UpstreamError> {
        let url = self.base.join(&format!("item/{id}.json"))?;
        Ok(self
            .http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
}
