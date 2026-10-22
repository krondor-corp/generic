//! Enqueues the periodic poll.
//!
//! A `Service` with a `tokio::time::interval` that pushes a
//! [`PollStoriesTask`] each tick (the first tick fires immediately, so a
//! fresh deploy populates itself). Keeping "when" here and "what" in the
//! task means a poll can also be enqueued by hand — from an admin route,
//! a CLI, a test — without duplicating the scraping logic.

use tokio::sync::watch;

use crate::runtime;
use crate::state::AppState;
use crate::tasks::PollStoriesTask;

pub struct Scheduler;

#[async_trait::async_trait]
impl runtime::Service for Scheduler {
    type State = AppState;

    async fn run(state: Self::State, mut shutdown_rx: watch::Receiver<()>) {
        let mut ticker = tokio::time::interval(state.poll_interval);
        ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        tracing::info!(interval = ?state.poll_interval, "scheduler started");

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    let task = PollStoriesTask { limit: state.poll_limit };
                    if let Err(e) = state.tasks.push(task).await {
                        tracing::error!("failed to enqueue poll: {e}");
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
