//! In-process event bus: a broadcast channel of scoped envelopes.
//!
//! The application defines the event enum (`AppEvent`) and aliases the
//! generic types:
//!
//! ```rust,ignore
//! pub type EventBus = generic_rust_core::events::EventBus<AppEvent>;
//! pub type Envelope = generic_rust_core::events::Envelope<AppEvent>;
//! pub use generic_rust_core::events::Scope;
//! ```
//!
//! Producers (handlers, tasks) call `events.publish(Envelope { scope, event })`.
//! Consumers (SSE endpoints) `subscribe()` and filter with [`Scope::permits`].

use tokio::sync::broadcast;
use uuid::Uuid;

/// Who an event is for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Scope {
    User(Uuid),
    Broadcast,
}

impl Scope {
    /// Whether a subscriber acting as `user_id` should receive this event.
    pub fn permits(&self, user_id: Uuid) -> bool {
        match self {
            Scope::Broadcast => true,
            Scope::User(id) => *id == user_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Envelope<E> {
    pub scope: Scope,
    pub event: E,
}

impl<E> Envelope<E> {
    pub fn broadcast(event: E) -> Self {
        Self {
            scope: Scope::Broadcast,
            event,
        }
    }

    pub fn user(user_id: Uuid, event: E) -> Self {
        Self {
            scope: Scope::User(user_id),
            event,
        }
    }
}

/// Broadcast channel of [`Envelope<E>`]. Cheap to clone; dropping every
/// receiver is fine — `publish` silently discards when nobody listens.
pub struct EventBus<E> {
    tx: broadcast::Sender<Envelope<E>>,
}

impl<E> Clone for EventBus<E> {
    fn clone(&self) -> Self {
        Self {
            tx: self.tx.clone(),
        }
    }
}

impl<E: Clone + Send + 'static> EventBus<E> {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn publish(&self, envelope: Envelope<E>) {
        let _ = self.tx.send(envelope);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Envelope<E>> {
        self.tx.subscribe()
    }
}
