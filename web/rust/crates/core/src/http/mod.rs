//! HTTP pieces every service mounts. Today that is the `/_status`
//! health router; applications build their own `HttpServer` service
//! around it (see `web/rust/crates/app/src/http/mod.rs`).

pub mod health;
