//! `GET /api/v0/stories` — mirrors `mobile_api::stories::StoriesRequest`.

use axum::extract::{Query, State};
use axum::Json;
use mobile_api::stories::{StoriesRequest, StoriesResponse, DEFAULT_LIMIT, MAX_LIMIT};

use crate::database::models::Story;
use crate::http::auth::RequireBearer;
use crate::http::error::ApiError;
use crate::state::AppState;

pub async fn handler(
    RequireBearer: RequireBearer,
    State(state): State<AppState>,
    Query(query): Query<StoriesRequest>,
) -> Result<Json<StoriesResponse>, ApiError> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

    let stories = Story::list_since(query.since, limit, &state.database)
        .await?
        .iter()
        .map(Story::to_wire)
        .collect();
    let cursor = Story::max_seq(&state.database).await?;

    Ok(Json(StoriesResponse { stories, cursor }))
}
