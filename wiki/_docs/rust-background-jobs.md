---
title: Background Jobs
order: 14
---

Background jobs in the Rust services use [Apalis](https://github.com/geofmureithi/apalis), a Rust job framework, with its SQLite backend. The queue is a `Jobs` table in the same database as everything else, so a job is enqueued in the same transaction scope as the row it belongs to and there is no broker to run.

The pieces are split between the two crates:

- **`generic-rust-core::tasks`** (shared): the `Task` trait, `push`, `TaskProducer`, and `run_monitor`.
- **The app's `tasks/`**: a `TaskContext` struct (what handlers get injected), one file per task type, and a `TaskWorker` service that registers them all.

## The Task trait

```rust
pub trait Task: Serialize + DeserializeOwned + Send + Sync + Unpin + 'static {
    /// What the handler needs injected via `.data(..)` — typically a
    /// struct of `Database` + `EventBus`.
    type Context: Clone + Send + Sync + 'static;

    /// Worker name for logging/monitoring (e.g. "process-widget-worker").
    const WORKER_NAME: &'static str;

    /// Register this task's worker with the monitor.
    /// Each impl creates its own `SqliteStorage` and `WorkerBuilder`.
    fn register(monitor: Monitor, ctx: Self::Context) -> Monitor;
}
```

A task is a plain `Serialize + Deserialize` struct; Apalis stores it as JSON in `Jobs` with `job_type` set to the Rust type name, so every task type shares one table but gets its own `SqliteStorage<T>` and named worker. Enqueueing is the same for every type, so it is not on the trait: `generic_rust_core::tasks::push(task, &db)`, or the `TaskProducer` handle that apps keep on their state.

## Defining a task

One file per task type under `tasks/`. This is the site's `tasks/widget/process.rs`, trimmed:

```rust
use apalis::prelude::*;
use apalis_sqlite::SqliteStorage;
use serde::{Deserialize, Serialize};

use crate::tasks::{Task, TaskContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessWidgetTask {
    pub widget_id: Uuid,
}

impl Task for ProcessWidgetTask {
    type Context = TaskContext;

    const WORKER_NAME: &'static str = "process-widget-worker";

    fn register(monitor: Monitor, ctx: TaskContext) -> Monitor {
        let storage = SqliteStorage::<Self, (), ()>::new(&ctx.db);

        monitor.register(move |_| {
            WorkerBuilder::new(Self::WORKER_NAME)
                .backend(storage.clone())
                .data(ctx.db.clone())
                .data(ctx.events.clone())
                .build(handler)
        })
    }
}

pub async fn handler(
    task: ProcessWidgetTask,
    db: Data<Database>,
    events: Data<EventBus>,
) -> Result<(), BoxDynError> {
    let widget = Widget::find(task.widget_id, &db).await?
        .ok_or_else(|| format!("widget {} not found", task.widget_id))?;
    // ... do the work, publish progress on `events`
    Ok(())
}
```

Everything the handler needs arrives as `Data<T>` extractors, matching the `.data(..)` calls in `register`. Returning `Err` marks the job failed; Apalis retries with its default policy.

## The app's side: context and worker

`tasks/mod.rs` defines what gets injected and the service that runs the monitor:

```rust
pub use generic_rust_core::tasks::{Task, TaskProducer};

#[derive(Clone)]
pub struct TaskContext {
    pub db: Database,
    pub events: EventBus,
}

pub struct TaskWorker;

#[async_trait::async_trait]
impl runtime::Service for TaskWorker {
    type State = AppState;

    async fn run(state: AppState, shutdown_rx: watch::Receiver<()>) {
        let ctx = TaskContext { db: state.database.clone(), events: state.events.clone() };

        let monitor = Monitor::new();
        let monitor = ProcessWidgetTask::register(monitor, ctx.clone());
        // one line per task type

        generic_rust_core::tasks::run_monitor(monitor, shutdown_rx).await;
    }
}
```

`main.rs` spawns it next to the HTTP server:

```rust
let (mut handle, shutdown_rx) = ShutdownHandle::new();
handle.push(HttpServer::spawn(state.clone(), shutdown_rx.clone()));
handle.push(TaskWorker::spawn(state.clone(), shutdown_rx.clone()));
handle.wait().await;
```

`run_monitor` runs the monitor until `shutdown_rx` fires, then returns; in-flight jobs stay in `Jobs` and are picked up on the next start. Adding a task type is the struct, its `impl Task`, and one `register` line here.

## Enqueueing

`AppState` carries a `TaskProducer` built from the pool. Handlers, schedulers, and other tasks push through it:

```rust
// http/html/widgets/actions/process.rs
if let Err(e) = state.tasks.push(ProcessWidgetTask { widget_id }).await {
    tracing::error!(error = %e, "failed to enqueue");
}
```

The row is in `Jobs` when `push` returns. The worker polls SQLite on a coarse interval (and processes in bursts), so expect a short delay rather than immediate pickup. This is also how the mobile server's `Scheduler` works: a plain tokio ticker that pushes a `HeartbeatTask` every interval, so the schedule is one place and the work is a normal job.

## Migrations

The Apalis tables are created by migrations that live in the app's `migrations/` directory, alongside its own, because one database has one `_sqlx_migrations` table. A new service copies the Apalis migration files from `web/rust/crates/app/migrations/` into its own directory.

## Keeping the table small

Done jobs are not deleted by Apalis. The mobile server shows the pattern: a `PruneTask` on its own ticker that deletes `Jobs` rows with `status = 'Done'` older than a day, next to whatever domain pruning the app needs. The site does not do this yet.

## Version pin

Apalis is pinned to `=1.0.0-rc.7` and `apalis-sqlite` to `=1.0.0-rc.6` in both workspaces. 1.0.0 stable changed the builder and storage API; upgrading means touching `register` in every task and `push` in core at the same time.
