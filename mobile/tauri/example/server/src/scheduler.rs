//! Enqueues the periodic jobs: the heartbeat on `POLL_INTERVAL_SECS`, the
//! prune on `PRUNE_INTERVAL_SECS`.
//!
//! A `Service` with one `tokio::time::interval` per job (the first tick of
//! each fires immediately, so a fresh deploy has a row within seconds).
//! Keeping "when" here and "what" in the tasks means either job can also
//! be enqueued by hand — from an admin route, a CLI, a test — without
//! duplicating its logic. Add a job: add a ticker and a `select!` arm.

use tokio::sync::watch;

use crate::runtime;
use crate::state::AppState;
use crate::tasks::{HeartbeatTask, PruneTask};

pub struct Scheduler;

#[async_trait::async_trait]
impl runtime::Service for Scheduler {
    type State = AppState;

    async fn run(state: Self::State, mut shutdown_rx: watch::Receiver<()>) {
        let mut heartbeat = tokio::time::interval(state.poll_interval);
        heartbeat.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        let mut prune = tokio::time::interval(state.prune_interval);
        prune.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        tracing::info!(
            heartbeat = ?state.poll_interval,
            prune = ?state.prune_interval,
            keep = state.retain_entries,
            "scheduler started"
        );

        let mut tick: u64 = 0;
        loop {
            tokio::select! {
                _ = heartbeat.tick() => {
                    tick += 1;
                    if let Err(e) = state.tasks.push(HeartbeatTask { tick }).await {
                        tracing::error!("failed to enqueue heartbeat: {e}");
                    }
                }
                _ = prune.tick() => {
                    let task = PruneTask { keep: state.retain_entries };
                    if let Err(e) = state.tasks.push(task).await {
                        tracing::error!("failed to enqueue prune: {e}");
                    }
                }
                _ = shutdown_rx.changed() => {
                    tracing::info!("shutting down scheduler");
                    return;
                }
            }
        }
    }
}
