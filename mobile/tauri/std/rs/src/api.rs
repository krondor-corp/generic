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
}

impl ServerConfig {
    pub async fn load(settings: &Settings) -> Result<Option<Self>> {
        Ok(settings
            .get(keys::SERVER_URL)
            .await?
            .filter(|url| !url.is_empty())
            .map(|url| Self { url }))
    }

    pub async fn save(&self, settings: &Settings) -> Result<()> {
        // Validate before persisting so a typo surfaces in the form, not
        // on the next sync.
        Url::parse(&self.url)?;
        settings.set(keys::SERVER_URL, &self.url).await?;
        Ok(())
    }

    /// Write `url` only if nothing is saved yet, so a fresh install points
    /// somewhere useful without a setup screen and a user's override sticks.
    pub async fn seed_default(settings: &Settings, url: &str) -> Result<()> {
        if Self::load(settings).await?.is_none() {
            Self {
                url: url.to_string(),
            }
            .save(settings)
            .await?;
        }
        Ok(())
    }

    pub fn client(&self) -> Result<Client> {
        let base = Url::parse(&self.url)?;
        Ok(Client::new(&base)?)
    }
}

/// Load settings and build a client, or `NotConfigured`.
pub async fn client(settings: &Settings) -> Result<Client> {
    ServerConfig::load(settings)
        .await?
        .ok_or(Error::NotConfigured("server url"))?
        .client()
}
