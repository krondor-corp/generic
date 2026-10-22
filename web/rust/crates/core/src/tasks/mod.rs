//! Background jobs on Apalis + SQLite.
//!
//! Each task type implements [`Task`]: it names its worker and registers
//! itself (storage + `WorkerBuilder` + handler) with the Apalis `Monitor`.
//! Enqueueing is uniform, so [`push`] is a free function and
//! [`TaskProducer`] is the handle applications put on their state.
//!
//! The application composes its own worker service — register every
//! task type, then hand the monitor to [`run_monitor`]:
//!
//! ```rust,ignore
//! #[async_trait::async_trait]
//! impl runtime::Service for TaskWorker {
//!     type State = AppState;
//!     async fn run(state: AppState, shutdown_rx: watch::Receiver<()>) {
//!         let ctx = TaskContext { db: state.database.clone(), events: state.events.clone() };
//!         let monitor = Monitor::new();
//!         let monitor = ProcessWidgetTask::register(monitor, ctx.clone());
//!         run_monitor(monitor, shutdown_rx).await;
//!     }
//! }
//! ```
//!
//! Apalis stores all task types in the same `Jobs` table, distinguished
//! by `job_type` (the Rust type name). Each task type gets its own
//! `SqliteStorage<T>` and named worker. The Apalis migrations ship with
//! the application's migration directory (see `web/rust/crates/app/migrations`).

use apalis::prelude::*;
use apalis_sqlite::SqliteStorage;
use serde::{de::DeserializeOwned, Serialize};
use tokio::sync::watch;

use crate::database::Database;

/// A self-registering background task type.
pub trait Task: Serialize + DeserializeOwned + Send + Sync + Unpin + 'static {
    /// Whatever the handler needs injected via `.data(..)` — typically a
    /// struct of `Database` + `EventBus`.
    type Context: Clone + Send + Sync + 'static;

    /// Worker name for logging/monitoring (e.g. "process-widget-worker").
    const WORKER_NAME: &'static str;

    /// Register this task's worker with the monitor.
    /// Each impl creates its own `SqliteStorage` and `WorkerBuilder`.
    fn register(monitor: Monitor, ctx: Self::Context) -> Monitor;
}

/// Enqueue `task` for its worker.
pub async fn push<T: Task>(task: T, db: &Database) -> Result<(), anyhow::Error> {
    let mut storage = SqliteStorage::<T, (), ()>::new(db);
    storage
        .push(task)
        .await
        .map_err(|e| anyhow::anyhow!("failed to enqueue {}: {e}", T::WORKER_NAME))?;
    Ok(())
}

/// Handle for enqueueing tasks from handlers and other tasks.
#[derive(Clone)]
pub struct TaskProducer {
    db: Database,
}

impl TaskProducer {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    pub async fn push<T: Task>(&self, task: T) -> Result<(), anyhow::Error> {
        push(task, &self.db).await
    }
}

/// Run a fully-registered monitor until `shutdown_rx` fires.
pub async fn run_monitor(monitor: Monitor, mut shutdown_rx: watch::Receiver<()>) {
    let monitor = monitor.on_event(|ctx, event| {
        tracing::debug!(worker = ctx.name(), ?event, "worker event");
    });

    tracing::info!("task workers started");

    tokio::select! {
        result = monitor.run() => {
            if let Err(e) = result {
                tracing::error!("task monitor error: {e}");
            }
        }
        _ = shutdown_rx.changed() => {
            tracing::info!("shutting down task workers");
        }
    }
}
