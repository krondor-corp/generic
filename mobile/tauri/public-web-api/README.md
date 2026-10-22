# public-web-api

The example app: a server scrapes a public API on a schedule; the app mirrors the feed into on-device SQLite, polls for anything new while it is running, and fires a local notification when something arrives.

- **`server/`** — Axum + SQLite + Apalis on `generic-rust-core`. Scheduler → `PollStoriesTask` → Hacker News top stories over plain `reqwest` → `stories` table → `StoryEvent::New` on the event bus. Serves a marketing page at `/` (Askama, links to TestFlight once `TESTFLIGHT_URL` is set) and `GET /api/v0/stories` (cursor-paged), `POST /api/v0/devices`, `GET /api/v0/events` (SSE), all open — it is a free public API. Deployed as `ios` at `ios.generic.krondor.org` (`config/deploy/ios.yml`).
- **`app/`** — Tauri 2. `src-tauri/` opens the device DB with `mobile-std`, runs a 60s `Poller` calling `sync::run`, and exposes `get_status`, `save_settings`, `test_connection`, `reset_data`, `list_stories`, `sync_now`, `open_story`. `src/` is a two-tab React UI (Stories, Settings).

## Run it

```bash
cd mobile/tauri
make dev-server          # terminal 1: http://localhost:9100
make dev                 # terminal 2: desktop app
```

Debug builds (`make dev`, `make dev-ios`, `make dev-android`) point at the local server out of the box; the Stories tab fills on the first sync. Production builds default to `https://ios.generic.krondor.org`, and Settings overrides either. Lower `POLL_INTERVAL_SECS` on the server to watch "new story" notifications arrive.

Server env and routes: see `server/src/config.rs` and the wiki's *Mobile → API server*. Device-side patterns: `../std/README.md`.

## Where things are decided

| Decision | Where |
|----------|-------|
| What to scrape, how often | `server/src/config.rs` (`POLL_*`, `UPSTREAM_BASE_URL`), `server/src/upstream/hn.rs`, `server/src/tasks/stories/poll.rs` |
| Wire shapes | `../std/api/src/{stories,devices,events,health}` |
| Device schema | `app/src-tauri/migrations/` |
| Sync + notification rules | `app/src-tauri/src/sync.rs` |
| What the webview may do | `app/src-tauri/capabilities/default.json` |
| Poll interval on device | `SYNC_INTERVAL` in `app/src-tauri/src/lib.rs` |
