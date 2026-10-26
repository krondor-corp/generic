use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("database: {0}")]
    Database(#[from] sqlx::Error),
    #[error("database setup: {0}")]
    DatabaseSetup(#[from] generic_rust_core::database::DatabaseSetupError),
    #[error("api: {0}")]
    Api(#[from] mobile_api::ApiError),
    #[error("invalid url: {0}")]
    Url(#[from] url::ParseError),
    #[error("tauri: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("notification: {0}")]
    Notification(#[from] tauri_plugin_notification::Error),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("not configured: {0}")]
    NotConfigured(&'static str),
    #[error("{0}")]
    Other(String),
}

/// Tauri commands return `Result<T, E: Serialize>`; the frontend gets
/// the display string.
impl Serialize for Error {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}
