use std::net::SocketAddr;

use axum::extract::FromRef;
use generic_rust_core::Version;

use crate::config::Config;
use crate::database::{Database, MIGRATOR};
use crate::events::EventBus;
use crate::tasks::TaskProducer;
use crate::version::VERSION;

#[derive(Clone)]
pub struct AppState {
    pub database: Database,
    pub listen_address: SocketAddr,
    pub host_name: String,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub service_secret: String,
    pub events: EventBus,
    pub tasks: TaskProducer,
}

#[derive(Debug, thiserror::Error)]
pub enum AppStateSetupError {
    #[error("database setup failed: {0}")]
    Database(#[from] crate::database::DatabaseSetupError),
}

impl AppState {
    pub async fn from_config(config: &Config) -> Result<Self, AppStateSetupError> {
        let database = Database::connect(&config.sqlite_database_url, &MIGRATOR).await?;
        let events = EventBus::new(256);
        let tasks = TaskProducer::new(database.clone());

        Ok(Self {
            database,
            listen_address: config.listen_address,
            host_name: config.host_name.clone(),
            google_client_id: config.google_client_id.clone(),
            google_client_secret: config.google_client_secret.clone(),
            service_secret: config.service_secret.clone(),
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
