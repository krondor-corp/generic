---
title: Server
order: 19
---

`mobile/tauri/example/server` is the backend for the example app, deployed as the `ios` service at `ios.generic.krondor.org`. One domain does three jobs: a page at `/`, health under `/_status`, and the device API under `/api/v0`. It has the same architecture as the [Rust web service](rust), both built on `generic-rust-core` (`web/rust/crates/core`), minus accounts: there is nothing to sign into.

It is deliberately the minimum that still shows every moving part.

## What it does

1. A **`Scheduler`** service ticks every `POLL_INTERVAL_SECS` and enqueues a `HeartbeatTask` (the first tick is immediate, so a fresh deploy has a row within seconds).
2. The **task** writes one row to the `entries` table. Apalis persists the job in SQLite, so a restart does not lose it, and the handler gets its dependencies injected. The body of the task is the part you replace: poll a third-party API, run a report, send digests.
3. A second scheduled job, `PruneTask`, keeps the newest `RETAIN_ENTRIES` rows and drops Apalis's records of finished jobs, so the database stays small. Two tickers in one `Scheduler`; add a third the same way.
4. **One route** serves the table to devices from a cursor. No auth: it is a free public API.
5. The **page** renders the most recent rows from the same table, so the site is a live view of what the app syncs, and links to the TestFlight build once `TESTFLIGHT_URL` is set.

## Routes

| Route | Mirrors | Notes |
|-------|---------|-------|
| `GET /` | — | Askama page (`http/html/index.rs`); same Franken UI look as `web/rust` |
| `GET /static/*path` | — | `server/static/`, embedded with `rust-embed` in release builds |
| `GET /_status/{livez,readyz,version}` | `mobile_api::health` | from `generic_rust_core::http::health` |
| `GET /api/v0/entries?since=&limit=` | `mobile_api::entries::EntriesRequest` | rows with `seq > since`, oldest first; no `since` → most recent `limit` |

CORS is enabled on `/api/v0` only. The request types in `mobile-api` document the route they expect; keep paths in lockstep when adding endpoints.

## Layout

```
server/
  src/
    main.rs          # config → logging → state → spawn HttpServer, TaskWorker, Scheduler
    config.rs        # env: LISTEN_ADDRESS, SQLITE_DATABASE_URL, POLL_INTERVAL_SECS, PRUNE_INTERVAL_SECS, RETAIN_ENTRIES, TESTFLIGHT_URL
    state.rs         # AppState + FromRef impls for Database and Version
    scheduler.rs     # Service: one interval per job → tasks.push(HeartbeatTask / PruneTask)
    tasks/           # TaskContext, TaskWorker, heartbeat.rs, prune.rs
    database/        # MIGRATOR + models (Entry)
    http/            # HttpServer, error (JSON), api/entries/list.rs, html/{index,static_files}.rs
  templates/         # layouts/base.html, pages/index.html (Askama, compiled in)
  static/            # style.css (shared with web/rust), app-icon.png
  migrations/        # entries + copied Apalis migrations
  Dockerfile         # context is the repo root (depends on web/rust/crates/core by path)
```

## Configuration

| Env | Default | |
|-----|---------|-|
| `LISTEN_ADDRESS` | `127.0.0.1:9100` | |
| `SQLITE_DATABASE_URL` | `sqlite://data/app.db` | |
| `POLL_INTERVAL_SECS` | `300` | how often the heartbeat job runs |
| `PRUNE_INTERVAL_SECS` | `3600` | how often the prune job runs |
| `RETAIN_ENTRIES` | `100` | how many entries the prune job keeps |
| `TESTFLIGHT_URL` | unset | public TestFlight link; the page says "coming soon" until set |
| `LOG_LEVEL` | `info` | |

In production `TESTFLIGHT_URL` comes from `services.ios.testflight_url` in `confit.toml`. Leave it empty until the build exists, then fill it in and redeploy.

## Local development

```bash
cd mobile/tauri
make dev-server                     # :9100, a row every 120s
open http://localhost:9100          # the page
curl 'localhost:9100/api/v0/entries?limit=5'
```

Debug builds of the app already point at `http://localhost:9100` (`http://10.0.2.2:9100` on the Android emulator).

## Deployment

```bash
make infra ARGS="apply"             # once: adds the ios DNS record (bin/iac reads confit services)
make kamal ARGS="ios setup"         # first time
make kamal ARGS="ios deploy"
```

CD deploys automatically on pushes to `main` that touch the server, `mobile-api`, the shared `core` crate, or the deploy config (see `.github/workflows/cd.yml`).

Unlike the `web/` services, the Docker build context is the **repo root**, because the server depends on `web/rust/crates/core` by path. `.dockerignore` at the root keeps that context small.
