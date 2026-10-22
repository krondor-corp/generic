//! Build a [`mobile_api::Client`] from saved settings.
//!
//! The client is rebuilt per call rather than cached so a settings change
//! takes effect immediately; reqwest clients are cheap to construct.

use mobile_api::{Client, Url};

use crate::settings::{keys, Settings};
use crate::{Error, Result};

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerConfig {
    pub url: String,
    pub token: String,
}

impl ServerConfig {
    pub async fn load(settings: &Settings) -> Result<Option<Self>> {
        let url = settings.get(keys::SERVER_URL).await?;
        let token = settings.get(keys::SERVER_TOKEN).await?;
        Ok(match (url, token) {
            (Some(url), Some(token)) if !url.is_empty() => Some(Self { url, token }),
            _ => None,
        })
    }

    pub async fn save(&self, settings: &Settings) -> Result<()> {
        // Validate before persisting so a typo surfaces in the form, not
        // on the next sync.
        Url::parse(&self.url)?;
        settings.set(keys::SERVER_URL, &self.url).await?;
        settings.set(keys::SERVER_TOKEN, &self.token).await?;
        Ok(())
    }

    pub fn client(&self) -> Result<Client> {
        let base = Url::parse(&self.url)?;
        Ok(Client::with_bearer(&base, &self.token)?)
    }
}

/// Load settings and build an authed client, or `NotConfigured`.
pub async fn client(settings: &Settings) -> Result<Client> {
    ServerConfig::load(settings)
        .await?
        .ok_or(Error::NotConfigured("server url and token"))?
        .client()
}
