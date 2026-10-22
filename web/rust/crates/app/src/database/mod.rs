pub mod models;

use sqlx::migrate::Migrator;

pub use generic_rust_core::database::{types, Database, DatabaseSetupError};

/// This app's migrations (its own tables plus the Apalis job tables).
/// One migrator per database — see `generic_rust_core::database`.
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
