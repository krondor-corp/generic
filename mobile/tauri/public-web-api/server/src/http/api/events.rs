//! `GET /api/v0/events` — SSE stream of `mobile_api::events::StoryEvent`.
//!
//! Every event here is broadcast-scoped (one feed for all devices), so
//! there is no per-subscriber filtering yet; `Scope::permits` is where it
//! would go.

use std::convert::Infallible;

use axum::extract::State;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream::Stream;
use tokio_stream::wrappers::BroadcastStream;
use tokio_stream::StreamExt;

use crate::events::AppEvent;
use crate::http::auth::RequireBearer;
use crate::state::AppState;

pub async fn handler(
    RequireBearer: RequireBearer,
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.events.subscribe();

    let stream = BroadcastStream::new(rx).filter_map(|msg| {
        let envelope = msg.ok()?;
        match envelope.event {
            AppEvent::Story(event) => {
                let json = serde_json::to_string(&event).ok()?;
                Some(Ok(Event::default().event(event.name()).data(json)))
            }
        }
    });

    Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(std::time::Duration::from_secs(15))
            .text("ping"),
    )
}
