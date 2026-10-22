---
title: API server
order: 19
---

`mobile/tauri/public-web-api/server` is the backend for the example app, deployed as the `mobile-api` service. It is a JSON-only Axum service with the same architecture as the [Rust web service](rust), minus templates: both are built on `generic-rust-core` (`web/rust/crates/core`).

## What it does

1. A **`Scheduler`** service ticks every `POLL_INTERVAL_SECS` and enqueues a `PollStoriesTask` (the first tick is immediate, so a fresh deploy populates itself).
2. The **task** fetches Hacker News' top story ids over plain `reqwest`, skips ids already in the database, fetches the unknown items with bounded concurrency, inserts them, and broadcasts a `StoryEvent::New` on the event bus. Steady state is one upstream request per poll.
3. **Routes** serve the result to devices. No auth: it is a free public API.

Hacker News is just a convenient public JSON API: no auth, two endpoints, stable schema. Nothing in the code is specific to its hosting. Swap `upstream/hn.rs` for any other public API and keep the rest.

## Routes

| Route | Mirrors | Notes |
|-------|---------|-------|
| `GET /_status/{livez,readyz,version}` | `mobile_api::health` | from `generic_rust_core::http::health`, public |
| `GET /api/v0/stories?since=&limit=` | `mobile_api::stories::StoriesRequest` | rows with `seq > since`, oldest first; no `since` → most recent `limit` |
| `POST /api/v0/devices` | `mobile_api::devices::RegisterDeviceRequest` | idempotent upsert on `device_id` |
| `GET /api/v0/events` | `mobile_api::events::StoryEvent` | SSE; event name `story_new` |

All `/api/v0` routes are open and read-only apart from device registration. The request types in `mobile-api` document the route they expect; keep paths in lockstep when adding endpoints.

## Layout

```
server/
  src/
    main.rs          # config → logging → state → spawn HttpServer, TaskWorker, Scheduler
    config.rs        # env: LISTEN_ADDRESS, SQLITE_DATABASE_URL, POLL_*, UPSTREAM_BASE_URL
    state.rs         # AppState + FromRef impls for Database and Version
    scheduler.rs     # Service: interval → tasks.push(PollStoriesTask)
    upstream/hn.rs   # reqwest client for the public API
    tasks/           # TaskContext, TaskWorker, stories/poll.rs
    events/          # AppEvent::Story(StoryEvent) — payloads are mobile-api wire types
    database/        # MIGRATOR + models (Story, Device)
    http/            # HttpServer, error (JSON), api/<resource>/<handler>.rs
  migrations/        # own tables + copied Apalis migrations
  Dockerfile         # context is the repo root (depends on web/rust/crates/core by path)
```

## Configuration

| Env | Default | |
|-----|---------|-|
| `LISTEN_ADDRESS` | `127.0.0.1:9100` | |
| `SQLITE_DATABASE_URL` | `sqlite://data/app.db` | |
| `POLL_INTERVAL_SECS` | `300` | |
| `POLL_LIMIT` | `30` | top-N ids considered per poll |
| `UPSTREAM_BASE_URL` | `https://hacker-news.firebaseio.com/v0/` | trailing slash matters |
| `LOG_LEVEL` | `info` | |

## Local development

```bash
cd mobile/tauri
make dev-server                     # :9100, polls every 120s
curl 'localhost:9100/api/v0/stories?limit=5'
```

Debug builds of the app already point at `http://localhost:9100` (`http://10.0.2.2:9100` on the Android emulator). From a physical device set your machine's LAN address in Settings.

## Deployment

```bash
make infra ARGS="apply"             # once: adds the mobile-api DNS record (bin/iac reads confit services)
make kamal ARGS="mobile-api setup"  # first time
make kamal ARGS="mobile-api deploy"
```

CD deploys automatically on pushes to `main` that touch the server, `mobile-api`, the shared `core` crate, or the deploy config (see `.github/workflows/cd.yml`).

Unlike the `web/` services, the Docker build context is the **repo root**, because the server depends on `web/rust/crates/core` by path. `.dockerignore` at the root keeps that context small.
