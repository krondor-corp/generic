//! The cleanup job: keep the database small.
//!
//! Runs on its own schedule (`PRUNE_INTERVAL_SECS`), independent of the
//! heartbeat, and keeps the newest `RETAIN_ENTRIES` rows. It also drops
//! Apalis's own records of finished jobs, which otherwise grow by one row
//! per tick forever — the storage crate has no retention of its own.

use apalis::prelude::*;
use apalis_sqlite::SqliteStorage;
use serde::{Deserialize, Serialize};

use crate::database::models::Entry;
use crate::database::Database;
use crate::tasks::{Task, TaskContext};

/// How long to keep Apalis's record of a finished job, in seconds.
const RETAIN_DONE_JOBS_SECS: i64 = 24 * 60 * 60;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PruneTask {
    /// How many entries survive.
    pub keep: u32,
}

impl Task for PruneTask {
    type Context = TaskContext;

    const WORKER_NAME: &'static str = "prune-worker";

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

pub async fn handler(task: PruneTask, db: Data<Database>) -> Result<(), BoxDynError> {
    let entries = Entry::prune(task.keep, &db).await?;
    let jobs = prune_done_jobs(RETAIN_DONE_JOBS_SECS, &db).await?;
    tracing::info!(keep = task.keep, entries, jobs, "pruned");
    Ok(())
}

/// Delete Apalis job rows that finished more than `older_than_secs` ago.
/// The `Jobs` table comes from the copied Apalis migrations.
async fn prune_done_jobs(older_than_secs: i64, db: &Database) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "DELETE FROM Jobs WHERE status = 'Done' AND done_at < strftime('%s', 'now') - ?",
    )
    .bind(older_than_secs)
    .execute(&**db)
    .await?;
    Ok(result.rows_affected())
}
