mod data_source;
pub mod liveness;
mod readiness;
pub mod version;

use axum::extract::FromRef;
use axum::routing::get;
use axum::Router;
use http::header::{ACCEPT, ORIGIN};
use http::Method;
use tower_http::cors::{Any, CorsLayer};
use tower_http::limit::RequestBodyLimitLayer;

use crate::database::Database;
use crate::Version;

/// Healthcheck endpoints generally shouldn't contain anything other than headers which are counted
/// among these bytes in the limit. Large requests here should always be rejected.
const HEALTHCHECK_REQUEST_SIZE_LIMIT: usize = 1_024;

/// `/livez`, `/readyz`, `/version`. Mount with `.nest("/_status", health::router())`.
///
/// Generic over the application's state: it only needs to lend a
/// [`Database`] (for readiness) and a [`Version`] via `FromRef`.
pub fn router<S>() -> Router<S>
where
    S: Clone + Send + Sync + 'static,
    Database: FromRef<S>,
    Version: FromRef<S>,
{
    let cors_layer = CorsLayer::new()
        .allow_methods(vec![Method::GET])
        .allow_headers(vec![ACCEPT, ORIGIN])
        .allow_origin(Any)
        .allow_credentials(false);

    Router::new()
        .route("/livez", get(liveness::handler))
        .route("/readyz", get(readiness::handler))
        .route("/version", get(version::handler))
        .layer(cors_layer)
        .layer(RequestBodyLimitLayer::new(HEALTHCHECK_REQUEST_SIZE_LIMIT))
}
