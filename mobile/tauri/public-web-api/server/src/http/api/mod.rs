//! `/api/v0` — the routes `mobile-api` describes. One file per handler,
//! each exporting `handler`; the request docs in `mobile-api` name the
//! route they expect, keep these in lockstep.

pub mod devices;
pub mod events;
pub mod stories;

use axum::routing::{get, post};
use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/stories", get(stories::list::handler))
        .route("/devices", post(devices::register::handler))
        .route("/events", get(events::handler))
}
