pub mod config;
pub mod database;
pub mod events;
pub mod http;
pub mod state;
pub mod tasks;
pub(crate) mod version;

// Shared plumbing lives in generic-rust-core; re-exported so binaries and
// docs keep addressing it as `generic_rust_app::runtime` etc.
pub use generic_rust_core::{logging, runtime};
