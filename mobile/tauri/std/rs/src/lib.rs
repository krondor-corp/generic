//! On-device runtime shared by the Tauri apps.
//!
//! The app's `src-tauri/src/lib.rs` wires these together in `setup`:
//!
//! ```rust,ignore
//! let db = mobile_std::db::open(app.handle(), &MIGRATOR).await?;
//! let settings = mobile_std::settings::Settings::new(db.clone());
//! let identity = mobile_std::identity::Identity::load_or_create(&settings).await?;
//! let poller = mobile_std::poller::Poller::spawn(app.handle().clone(), "sync", interval, sync_once);
//! app.manage(AppState { db, settings, identity, poller });
//! ```
//!
//! Modules:
//!
//! - [`db`] — open + migrate SQLite under the platform app-data dir.
//! - [`settings`] — string KV table (`kv`) for server URL, cursors, flags.
//! - [`identity`] — stable per-install `device_id` + a display name.
//! - [`api`] — build a [`mobile_api::Client`] for the configured server.
//! - [`notify`] — local notifications with permission handling.
//! - [`poller`] — a named foreground loop: run `f` every `interval`, or
//!   now on `trigger()`, until `stop()`.
//! - [`Error`] — one error type, `Serialize`d as a string so it can be
//!   returned straight from `#[tauri::command]`s.

pub mod api;
pub mod db;
mod error;
pub mod identity;
pub mod notify;
pub mod poller;
pub mod settings;

pub use error::Error;
pub use generic_rust_core::database::{Database, DbUuid};

pub type Result<T> = std::result::Result<T, Error>;
