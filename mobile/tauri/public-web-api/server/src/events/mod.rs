use serde::Serialize;

pub use generic_rust_core::events::Scope;
pub use mobile_api::events::StoryEvent;

/// Everything this service can announce. The inner payloads are the
/// `mobile-api` wire types so the SSE stream and the device agree.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "domain", content = "event")]
pub enum AppEvent {
    Story(StoryEvent),
}

pub type Envelope = generic_rust_core::events::Envelope<AppEvent>;
pub type EventBus = generic_rust_core::events::EventBus<AppEvent>;
