//! `GET /` — the one marketing page. Renders the same rows the app
//! syncs (`Story::latest`) so the page doubles as a live demo of the API.

use askama::Template;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Html;

use crate::database::models::Story;
use crate::state::AppState;

/// How many recent stories the page shows.
const FEED_SIZE: u32 = 8;

#[derive(Template)]
#[template(path = "pages/index.html")]
struct IndexTemplate {
    testflight_url: Option<String>,
    stories: Vec<StoryRow>,
    story_count: i64,
    poll_interval_mins: u64,
}

/// A story pre-formatted for the template; keeps the template free of
/// logic Askama is bad at (URL parsing, relative time).
struct StoryRow {
    title: String,
    url: Option<String>,
    host: String,
    author: String,
    score: i64,
    age: String,
}

impl StoryRow {
    fn from_story(story: &Story, now: i64) -> Self {
        let host = story
            .url()
            .and_then(|u| url::Url::parse(u).ok())
            .and_then(|u| {
                u.host_str()
                    .map(|h| h.trim_start_matches("www.").to_string())
            })
            .unwrap_or_else(|| "text post".to_string());
        Self {
            title: story.title().to_string(),
            url: story.url().map(str::to_string),
            host,
            author: story.author().to_string(),
            score: story.score(),
            age: relative(now - story.posted_at()),
        }
    }
}

pub async fn handler(State(state): State<AppState>) -> Result<Html<String>, StatusCode> {
    let (latest, story_count) = tokio::try_join!(
        Story::latest(FEED_SIZE, &state.database),
        Story::count(&state.database),
    )
    .map_err(|e| {
        tracing::error!("index query failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let page = IndexTemplate {
        testflight_url: state.testflight_url.clone(),
        stories: latest
            .iter()
            .map(|s| StoryRow::from_story(s, now))
            .collect(),
        story_count,
        poll_interval_mins: state.poll_interval.as_secs().div_ceil(60).max(1),
    };

    page.render().map(Html).map_err(|e| {
        tracing::error!("index render failed: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

fn relative(secs: i64) -> String {
    let secs = secs.max(0);
    match secs {
        s if s < 60 => "just now".to_string(),
        s if s < 3600 => format!("{}m ago", s / 60),
        s if s < 86_400 => format!("{}h ago", s / 3600),
        s => format!("{}d ago", s / 86_400),
    }
}

#[cfg(test)]
mod tests {
    use super::relative;

    #[test]
    fn relative_buckets() {
        assert_eq!(relative(5), "just now");
        assert_eq!(relative(90), "1m ago");
        assert_eq!(relative(7_200), "2h ago");
        assert_eq!(relative(200_000), "2d ago");
        assert_eq!(relative(-10), "just now");
    }
}
