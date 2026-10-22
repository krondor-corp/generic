pub mod api;
pub mod error;
pub mod html;

use axum::Router;
use generic_rust_core::http::health;
use tokio::sync::watch;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::runtime;
use crate::state::AppState;

/// One domain, three surfaces: the marketing page at `/` (plus its
/// static files), health under `/_status`, and the device API under
/// `/api/v0`. CORS is only relevant to the API.
pub struct HttpServer;

#[async_trait::async_trait]
impl runtime::Service for HttpServer {
    type State = AppState;

    async fn run(state: Self::State, mut shutdown_rx: watch::Receiver<()>) {
        let addr = state.listen_address;

        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(Any);

        let app = Router::new()
            .merge(html::router())
            .nest("/_status", health::router())
            .nest("/api/v0", api::router().layer(cors))
            .with_state(state)
            .layer(TraceLayer::new_for_http());

        tracing::info!("listening on {addr}");

        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(e) => {
                tracing::error!("failed to bind {addr}: {e}");
                return;
            }
        };

        if let Err(e) = axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.changed().await;
            })
            .await
        {
            tracing::error!("server error: {e}");
        }
    }
}
