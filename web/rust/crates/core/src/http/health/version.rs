use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::Json;
use http::StatusCode;

use crate::Version;

#[tracing::instrument]
pub async fn handler(State(version): State<Version>) -> Response {
    (StatusCode::OK, Json(version)).into_response()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_handler_direct() {
        let response = handler(State(Version::new("test", "debug", "now"))).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}
