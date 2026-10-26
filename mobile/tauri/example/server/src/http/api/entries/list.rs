//! `GET /api/v0/entries` — mirrors `mobile_api::entries::EntriesRequest`.

use axum::extract::{Query, State};
use axum::Json;
use mobile_api::entries::{EntriesRequest, EntriesResponse, DEFAULT_LIMIT, MAX_LIMIT};

use crate::database::models::Entry;
use crate::http::error::ApiError;
use crate::state::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Query(query): Query<EntriesRequest>,
) -> Result<Json<EntriesResponse>, ApiError> {
    let limit = query.limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT);

    let entries = Entry::list_since(query.since, limit, &state.database)
        .await?
        .iter()
        .map(Entry::to_wire)
        .collect();
    let cursor = Entry::max_seq(&state.database).await?;

    Ok(Json(EntriesResponse { entries, cursor }))
}
