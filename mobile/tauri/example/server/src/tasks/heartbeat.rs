//! The one background job: write a row.
//!
//! It stands in for whatever a real server does on a schedule — poll a
//! third-party API, run a report, send digests. Everything around it is
//! the part worth copying: the task is a serialisable struct, Apalis
//! persists it in SQLite so a restart does not lose it, the handler gets
//! its dependencies injected, and the result lands in a table with a
//! monotonic `seq` that devices page on.

use apalis::prelude::*;
use apalis_sqlite::SqliteStorage;
use serde::{Deserialize, Serialize};

use crate::database::models::Entry;
use crate::database::Database;
use crate::tasks::{Task, TaskContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeartbeatTask {
    /// Which scheduler tick enqueued this; purely informational.
    pub tick: u64,
}

impl Task for HeartbeatTask {
    type Context = TaskContext;

    const WORKER_NAME: &'static str = "heartbeat-worker";

    fn register(monitor: Monitor, ctx: TaskContext) -> Monitor {
        let storage = SqliteStorage::<Self, (), ()>::new(&ctx.db);

        monitor.register(move |_| {
            WorkerBuilder::new(Self::WORKER_NAME)
                .backend(storage.clone())
                .data(ctx.db.clone())
                .build(handler)
        })
    }
}

pub async fn handler(task: HeartbeatTask, db: Data<Database>) -> Result<(), BoxDynError> {
    let entry = Entry::create(&format!("heartbeat #{}", task.tick), &db).await?;
    tracing::info!(seq = entry.seq(), tick = task.tick, "wrote entry");
    Ok(())
}
