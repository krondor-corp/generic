pub mod heartbeat;
pub mod prune;

use apalis::prelude::*;
use tokio::sync::watch;

use crate::database::Database;
use crate::runtime;
use crate::state::AppState;

pub use generic_rust_core::tasks::{Task, TaskProducer};
pub use heartbeat::HeartbeatTask;
pub use prune::PruneTask;

/// What every task handler in this service gets injected.
#[derive(Clone)]
pub struct TaskContext {
    pub db: Database,
}

/// Registers each task type's worker and runs the Apalis monitor.
pub struct TaskWorker;

#[async_trait::async_trait]
impl runtime::Service for TaskWorker {
    type State = AppState;

    async fn run(state: Self::State, shutdown_rx: watch::Receiver<()>) {
        let ctx = TaskContext {
            db: state.database.clone(),
        };

        let monitor = Monitor::new();
        let monitor = HeartbeatTask::register(monitor, ctx.clone());
        let monitor = PruneTask::register(monitor, ctx);

        generic_rust_core::tasks::run_monitor(monitor, shutdown_rx).await;
    }
}
