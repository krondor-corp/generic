use std::net::SocketAddr;
use std::time::Duration;

use url::Url;

#[derive(Debug)]
pub struct Config {
    pub listen_address: SocketAddr,
    pub sqlite_database_url: Url,
    pub log_level: tracing::Level,
    /// How often the scheduler enqueues the heartbeat job.
    pub poll_interval: Duration,
    /// How often the scheduler enqueues the prune job.
    pub prune_interval: Duration,
    /// How many entries the prune job keeps.
    pub retain_entries: u32,
    /// Public TestFlight link for the page. Unset (or empty) until the
    /// build exists; the page then says so instead of linking.
    pub testflight_url: Option<Url>,
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

        let prune_interval = Duration::from_secs(
            std::env::var("PRUNE_INTERVAL_SECS")
                .unwrap_or_else(|_| "3600".to_string())
                .parse()
                .map_err(|_| ConfigError::InvalidInt("PRUNE_INTERVAL_SECS"))?,
        );

        let retain_entries = std::env::var("RETAIN_ENTRIES")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .map_err(|_| ConfigError::InvalidInt("RETAIN_ENTRIES"))?;

        let testflight_url = match std::env::var("TESTFLIGHT_URL") {
            Ok(value) if !value.trim().is_empty() => Some(Url::parse(value.trim())?),
            _ => None,
        };

        Ok(Self {
            listen_address,
            sqlite_database_url,
            log_level,
            poll_interval,
            prune_interval,
            retain_entries,
            testflight_url,
        })
    }
}
