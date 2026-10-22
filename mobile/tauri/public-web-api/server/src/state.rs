use std::net::SocketAddr;
use std::time::Duration;

use axum::extract::FromRef;
use generic_rust_core::Version;

use crate::config::Config;
use crate::database::{Database, MIGRATOR};
use crate::events::EventBus;
use crate::tasks::TaskProducer;
use crate::upstream::HackerNews;
use crate::version::VERSION;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub listen_address: SocketAddr,
    pub poll_interval: Duration,
    pub poll_limit: usize,
    pub upstream: HackerNews,
    pub events: EventBus,
    pub tasks: TaskProducer,
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    #[error("database setup failed: {0}")]
    Database(#[from] crate::database::DatabaseSetupError),
    #[error("upstream client setup failed: {0}")]
    Upstream(#[from] reqwest::Error),
}

impl AppState {
    pub async fn from_config(config: &Config) -> Result<Self, AppStateSetupError> {
        let database = Database::connect(&config.sqlite_database_url, &MIGRATOR).await?;
        let events = EventBus::new(256);
        let tasks = TaskProducer::new(database.clone());
        let upstream = HackerNews::new(config.upstream_base_url.clone())?;

        Ok(Self {
            database,
            listen_address: config.listen_address,
            poll_interval: config.poll_interval,
            poll_limit: config.poll_limit,
            upstream,
            events,
            tasks,
        })
    }
}

impl FromRef<AppState> for Database {
    fn from_ref(state: &AppState) -> Self {
        state.database.clone()
    }
}

impl FromRef<AppState> for Version {
    fn from_ref(_: &AppState) -> Self {
        VERSION
    }
}
