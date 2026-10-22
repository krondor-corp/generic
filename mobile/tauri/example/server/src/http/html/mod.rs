//! The marketing site: one Askama page at `/` in the same style as
//! `web/rust`, plus embedded static files. No accounts — there is
//! nothing to sign into; the page exists to explain the app and link to
//! it.

mod index;
mod static_files;

use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(index::handler))
        .route("/static/*path", get(static_files::handler))
}
