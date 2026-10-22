//! Pull everything past our cursor from the server, store it, tell the
//! user. Called by the poller on its interval and by the `sync_now`
//! command; both go through [`run`] so the bookkeeping (last sync time,
//! last error, frontend events) is identical.

use mobile_api::stories::{StoriesRequest, DEFAULT_LIMIT};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Runtime};

use crate::database::models::Story;
use crate::state::{keys, AppState};

/// Guard against a runaway backlog in one go; the next tick picks up the rest.
const MAX_PAGES: usize = 10;

pub const EVENT_DONE: &str = "sync:done";
pub const EVENT_ERROR: &str = "sync:error";

#[derive(Debug, Clone, Serialize)]
pub struct SyncReport {
    pub fetched: usize,
    pub new_count: usize,
    pub cursor: i64,
}

/// One sync with bookkeeping + events. The poller's `f`.
pub async fn run<R: Runtime>(app: AppHandle<R>) -> mobile_std::Result<SyncReport> {
    let state = app.state::<AppState>();
    match sync_once(&app, &state).await {
        Ok(report) => {
            state
                .settings
                .set(keys::LAST_SYNC_AT, &now_rfc3339())
                .await?;
            state.settings.delete(keys::LAST_ERROR).await?;
            let _ = app.emit(EVENT_DONE, &report);
            Ok(report)
        }
        Err(e) => {
            let message = e.to_string();
            let _ = state.settings.set(keys::LAST_ERROR, &message).await;
            let _ = app.emit(EVENT_ERROR, &message);
            Err(e)
        }
    }
}

async fn sync_once<R: Runtime>(
    app: &AppHandle<R>,
    state: &AppState,
) -> mobile_std::Result<SyncReport> {
    let client = mobile_std::api::client(&state.settings).await?;

    // Register (or touch) this install first — cheap, idempotent, and it
    // means the server's device list reflects who is actually syncing.
    let register = state.identity.lock().await.register_request();
    client.call(register).await?;

    let starting_cursor = state.settings.get_i64(keys::CURSOR).await?;
    let mut since = starting_cursor;
    let mut fetched = 0;
    let mut new_titles: Vec<String> = Vec::new();
    let mut cursor = since.unwrap_or(0);

    for _ in 0..MAX_PAGES {
        let page = client
            .call(StoriesRequest {
                since,
                limit: Some(DEFAULT_LIMIT),
            })
            .await?;

        fetched += page.stories.len();
        for story in &page.stories {
            if Story::insert_if_new(story, &state.db).await? {
                new_titles.push(story.title.clone());
            }
        }

        cursor = page.cursor;
        let last_seq = page.stories.last().map(|s| s.seq);
        state.settings.set_i64(keys::CURSOR, cursor).await?;

        // Short page, or the server's cursor is behind what we just saw:
        // caught up.
        if page.stories.len() < DEFAULT_LIMIT as usize || last_seq.is_none_or(|s| s >= cursor) {
            break;
        }
        since = last_seq;
    }

    // A fresh install's first sync is a backfill, not news.
    if starting_cursor.is_some() && !new_titles.is_empty() {
        let title = match new_titles.len() {
            1 => "1 new story".to_string(),
            n => format!("{n} new stories"),
        };
        mobile_std::notify::notify(app, &title, &new_titles[0])?;
    }

    tracing::info!(fetched, new = new_titles.len(), cursor, "synced");
    Ok(SyncReport {
        fetched,
        new_count: new_titles.len(),
        cursor,
    })
}

fn now_rfc3339() -> String {
    // Avoid a chrono dependency for one timestamp: ask SQLite's format
    // sibling, the system clock, via std.
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Civil-from-days (Howard Hinnant), good for 1970..9999.
    let days = secs / 86_400;
    let rem = secs % 86_400;
    let (h, m, s) = (rem / 3600, (rem % 3600) / 60, rem % 60);
    let z = days as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let mo = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if mo <= 2 { y + 1 } else { y };
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

#[cfg(test)]
mod tests {
    use super::now_rfc3339;

    #[test]
    fn timestamp_shape() {
        let ts = now_rfc3339();
        assert_eq!(ts.len(), 20, "{ts}");
        assert!(ts.ends_with('Z'));
        assert!(ts.starts_with("20"));
    }
}
