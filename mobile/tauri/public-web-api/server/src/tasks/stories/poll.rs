//! Scrape upstream's top stories, store the ones we have not seen, and
//! announce each on the event bus.
//!
//! The list endpoint returns ids only, so this is a fan-out: one request
//! for the list, then one per *unknown* id with bounded concurrency.
//! Known ids are skipped entirely — we do not refresh scores — which
//! keeps steady-state polls down to a single upstream request.

use apalis::prelude::*;
use apalis_sqlite::SqliteStorage;
use futures::stream::{self, StreamExt};
use serde::{Deserialize, Serialize};

use crate::database::models::{NewStory, Story};
use crate::database::Database;
use crate::events::{AppEvent, Envelope, EventBus, StoryEvent};
use crate::tasks::{Task, TaskContext};
use crate::upstream::HackerNews;

/// Max simultaneous item fetches per poll.
const FETCH_CONCURRENCY: usize = 8;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PollStoriesTask {
    /// How many of the top list to consider.
    pub limit: usize,
}

impl Task for PollStoriesTask {
    type Context = TaskContext;

    const WORKER_NAME: &'static str = "poll-stories-worker";

    fn register(monitor: Monitor, ctx: TaskContext) -> Monitor {
        let storage = SqliteStorage::<Self, (), ()>::new(&ctx.db);

        monitor.register(move |_| {
            WorkerBuilder::new(Self::WORKER_NAME)
                .backend(storage.clone())
                .data(ctx.db.clone())
                .data(ctx.events.clone())
                .data(ctx.upstream.clone())
                .build(handler)
        })
    }
}

pub async fn handler(
    task: PollStoriesTask,
    db: Data<Database>,
    events: Data<EventBus>,
    upstream: Data<HackerNews>,
) -> Result<(), BoxDynError> {
    let ids: Vec<i64> = upstream
        .top_stories()
        .await?
        .into_iter()
        .take(task.limit)
        .collect();

    let known = Story::known_ids(&ids, &db).await?;
    let unknown: Vec<i64> = ids.into_iter().filter(|id| !known.contains(id)).collect();
    tracing::info!(
        considered = task.limit,
        known = known.len(),
        unknown = unknown.len(),
        "polled upstream"
    );

    if unknown.is_empty() {
        return Ok(());
    }

    let items = stream::iter(unknown)
        .map(|id| {
            let upstream = upstream.clone();
            async move {
                match upstream.item(id).await {
                    Ok(item) => item,
                    Err(e) => {
                        tracing::warn!(id, error = %e, "failed to fetch item");
                        None
                    }
                }
            }
        })
        .buffer_unordered(FETCH_CONCURRENCY)
        .filter_map(|item| async { item })
        .collect::<Vec<_>>()
        .await;

    let mut inserted = 0;
    for item in items.into_iter().filter(|i| i.is_story()) {
        let new = NewStory {
            id: item.id,
            title: item.title.unwrap_or_default(),
            url: item.url,
            author: item.by.unwrap_or_default(),
            score: item.score.unwrap_or(0),
            posted_at: item.time.unwrap_or(0),
        };
        // Another worker may have raced us; UNIQUE(id) makes that a no-op.
        match Story::create(new, &db).await {
            Ok(story) => {
                inserted += 1;
                events.publish(Envelope::broadcast(AppEvent::Story(StoryEvent::New {
                    story: story.to_wire(),
                })));
            }
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() => {}
            Err(e) => return Err(e.into()),
        }
    }

    tracing::info!(inserted, "poll complete");
    Ok(())
}
