//! `GET /` — the one page. Renders the same rows the app syncs
//! (`Entry::latest`) so the page doubles as a live view of the API.

use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use crate::database::models::Entry;
use crate::state::AppState;

/// How many recent entries the page shows.
const FEED_SIZE: u32 = 8;

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
    testflight_url: Option<String>,
    entries: Vec<EntryRow>,
    entry_count: i64,
    poll_interval_mins: u64,
}

/// An entry pre-formatted for the template.
struct EntryRow {
    seq: i64,
    body: String,
    created_at: String,
}

pub async fn handler(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let (latest, entry_count) = tokio::try_join!(
        Entry::latest(FEED_SIZE, &state.database),
        Entry::count(&state.database),
    )
    .map_err(|e| {
        tracing::error!("index query failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let page = IndexTemplate {
        testflight_url: state.testflight_url.clone(),
        entries: latest
            .iter()
            .map(|e| EntryRow {
                seq: e.seq(),
                body: e.body().to_string(),
                created_at: e.created_at().to_string(),
            })
            .collect(),
        entry_count,
        poll_interval_mins: state.poll_interval.as_secs().div_ceil(60).max(1),
    };

    page.render().map(Html).map_err(|e| {
        tracing::error!("index render failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}
