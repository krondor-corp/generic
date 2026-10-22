use std::net::SocketAddr;
use std::time::Duration;

use axum::extract::FromRef;
use generic_rust_core::Version;
use url::Url;

use crate::config::Config;
use crate::database::{Database, MIGRATOR};
use crate::tasks::TaskProducer;
use crate::version::VERSION;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub listen_address: SocketAddr,
    pub poll_interval: Duration,
    pub prune_interval: Duration,
    pub retain_entries: u32,
    pub tasks: TaskProducer,
    pub testflight_url: Option<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    #[error("database setup failed: {0}")]
    Database(#[from] crate::database::DatabaseSetupError),
}

impl AppState {
    pub async fn from_config(config: &Config) -> Result<Self, AppStateSetupError> {
        let database = Database::connect(&config.sqlite_database_url, &MIGRATOR).await?;
        let tasks = TaskProducer::new(database.clone());

        Ok(Self {
            database,
            listen_address: config.listen_address,
            poll_interval: config.poll_interval,
            prune_interval: config.prune_interval,
            retain_entries: config.retain_entries,
            tasks,
            testflight_url: config.testflight_url.as_ref().map(Url::to_string),
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
