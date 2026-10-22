//! Payloads on the server's SSE stream, `GET /api/v0/events` (RequireBearer).
//!
//! The stream is optional for devices — polling [`crate::stories`] is the
//! baseline — but when the app is foregrounded it can subscribe for
//! instant updates. Each SSE frame's `event:` name is [`StoryEvent::NAME`]
//! and its `data:` is the JSON of the variant.
//!
//! The server defines its own `AppEvent` enum and wraps this in it; only
//! the inner payload is on the wire.

use serde::{Deserialize, Serialize};

use crate::stories::Story;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum StoryEvent {
    New { story: Story },
}

impl StoryEvent {
    /// SSE `event:` name for this variant.
    pub fn name(&self) -> &'static str {
        match self {
            StoryEvent::New { .. } => "story_new",
        }
    }
}
