pub mod widget;

use serde::Serialize;

use widget::WidgetEvent;

pub use generic_rust_core::events::Scope;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "domain", content = "event")]
pub enum AppEvent {
    Widget(WidgetEvent),
}

pub type Envelope = generic_rust_core::events::Envelope<AppEvent>;
pub type EventBus = generic_rust_core::events::EventBus<AppEvent>;
