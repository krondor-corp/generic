pub mod stories;

use apalis::prelude::*;
use tokio::sync::watch;

use crate::database::Database;
use crate::events::EventBus;
use crate::runtime;
use crate::state::AppState;
use crate::upstream::HackerNews;

pub use generic_rust_core::tasks::{Task, TaskProducer};
pub use stories::PollStoriesTask;

/// What every task handler in this service gets injected.
#[derive(Clone)]
pub struct TaskContext {
    pub db: Database,
    pub events: EventBus,
    pub upstream: HackerNews,
}

/// Registers each task type's worker and runs the Apalis monitor.
pub struct TaskWorker;

#[async_trait::async_trait]
impl runtime::Service for TaskWorker {
    type State = AppState;

    async fn run(state: Self::State, shutdown_rx: watch::Receiver<()>) {
        let ctx = TaskContext {
            db: state.database.clone(),
            events: state.events.clone(),
            upstream: state.upstream.clone(),
        };

        let monitor = Monitor::new();
        let monitor = PollStoriesTask::register(monitor, ctx);

        generic_rust_core::tasks::run_monitor(monitor, shutdown_rx).await;
    }
}
