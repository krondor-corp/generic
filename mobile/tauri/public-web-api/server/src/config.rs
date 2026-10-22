use std::net::SocketAddr;
use std::time::Duration;

use url::Url;

#[derive(Debug)]
pub struct Config {
    pub listen_address: SocketAddr,
    pub sqlite_database_url: Url,
    pub log_level: tracing::Level,
    /// How often the scheduler enqueues a poll of upstream.
    pub poll_interval: Duration,
    /// How many of upstream's top stories to consider per poll.
    pub poll_limit: usize,
    /// Base of the upstream JSON API (trailing slash matters for joins).
    pub upstream_base_url: Url,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("invalid listen address: {0}")]
    InvalidAddress(#[from] std::net::AddrParseError),
    #[error("invalid url: {0}")]
    InvalidUrl(#[from] url::ParseError),
    #[error("invalid integer in {0}")]
    InvalidInt(&'static str),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let listen_address: SocketAddr = std::env::var("LISTEN_ADDRESS")
            .unwrap_or_else(|_| "127.0.0.1:9100".to_string())
            .parse()?;

        let sqlite_database_url = Url::parse(
            &std::env::var("SQLITE_DATABASE_URL")
                .unwrap_or_else(|_| "sqlite://data/app.db".to_string()),
        )?;

        let log_level = std::env::var("LOG_LEVEL")
            .unwrap_or_else(|_| "info".to_string())
            .parse()
            .unwrap_or(tracing::Level::INFO);

        let poll_interval = Duration::from_secs(
            std::env::var("POLL_INTERVAL_SECS")
                .unwrap_or_else(|_| "300".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidInt("POLL_INTERVAL_SECS"))?,
        );

        let poll_limit = std::env::var("POLL_LIMIT")
            .unwrap_or_else(|_| "30".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidInt("POLL_LIMIT"))?;

        let upstream_base_url = Url::parse(
            &std::env::var("UPSTREAM_BASE_URL")
                .unwrap_or_else(|_| "https://hacker-news.firebaseio.com/v0/".to_string()),
        )?;

        Ok(Self {
            listen_address,
            sqlite_database_url,
            log_level,
            poll_interval,
            poll_limit,
            upstream_base_url,
        })
    }
}
