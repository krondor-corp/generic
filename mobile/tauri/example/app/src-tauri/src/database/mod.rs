pub mod models;

use sqlx::migrate::Migrator;

pub use mobile_std::Database;

/// This app's device-side schema. `mobile_std::db::open` runs it.
pub static MIGRATOR: Migrator = sqlx::migrate!("./migrations");
