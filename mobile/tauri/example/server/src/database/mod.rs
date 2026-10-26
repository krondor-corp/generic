pub mod models;

use sqlx::migrate::Migrator;

pub use generic_rust_core::database::{types, Database, DatabaseSetupError};

/// This service's migrations (its own tables plus the Apalis job tables).
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
