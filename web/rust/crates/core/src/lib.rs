//! Shared runtime for the Rust services in this repo.
//!
//! Applications (`web/rust/crates/app`, `mobile/tauri/example/server`)
//! own their config, models, handlers, and tasks. This crate owns the
//! plumbing they have in common:
//!
//! - [`runtime`] — `Service` trait + `ShutdownHandle`.
//! - [`database`] — SQLite pool wrapper + `DbUuid`. Migrations are the
//!   application's: pass its `sqlx::migrate!()` to [`database::Database::connect`].
//! - [`events`] — `EventBus<E>` generic over the application's event enum.
//! - [`tasks`] — Apalis `Task` trait + `TaskProducer`.
//! - [`http`] — `/_status` health router.
//! - [`logging`] — tracing subscriber setup.
//! - [`Version`] — build metadata, populated from the application's `build.rs`.
//!
//! Every module is behind a cargo feature of the same name so lightweight
//! consumers (the on-device Tauri runtime) can take only `database`.

#[cfg(feature = "database")]
pub mod database;
#[cfg(feature = "events")]
pub mod events;
#[cfg(feature = "http")]
pub mod http;
#[cfg(feature = "logging")]
pub mod logging;
#[cfg(feature = "runtime")]
pub mod runtime;
#[cfg(feature = "tasks")]
pub mod tasks;

mod version;

pub use version::Version;
