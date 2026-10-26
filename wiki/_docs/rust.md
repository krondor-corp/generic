---
title: Overview
order: 12
---

The Rust service lives at `web/rust/` and is deployed via Kamal with the config at `config/deploy/rust.yml`.

## Overview

The Rust stack is for lightweight, self-contained apps that don't need a separate frontend build or JavaScript framework. It compiles to a single binary with zero runtime dependencies. Pages are server-rendered with Askama templates and enhanced with HTMX for interactivity and SSE for real-time updates.

`web/rust/` is a cargo workspace of two crates:

| Crate | Package | What it is |
|-------|---------|------------|
| `crates/core` | `generic-rust-core` | The runtime every Rust binary in the repo shares. Feature-gated; no application code. |
| `crates/app` | `generic-rust-app` | This site: config, models, handlers, templates, tasks, migrations. |

## The core crate

`generic-rust-core` is the plumbing that was common to the site and the [mobile server](mobile-api-server), pulled out so a new Rust service starts from a working runtime instead of a copy of this one. Each module is behind a cargo feature of the same name; the default is everything, and consumers opt down with `default-features = false, features = [...]`.

| Feature | Exports | Use it for |
|---------|---------|------------|
| `runtime` | `Service` trait, `ShutdownHandle` | Long-running pieces (HTTP server, worker, scheduler). Implement `run(state, shutdown_rx)`; `spawn()` comes free. The handle listens for SIGINT/SIGTERM, gives SIGTERM a 10 s grace period, then waits up to 30 s for services to exit. |
| `database` | `Database`, `DbUuid` | A connected, migrated SQLite pool. Derefs to `SqlitePool`. `Database::connect(url, &MIGRATOR)` for servers, `connect_file(path, &MIGRATOR)` where the platform picks the location (Tauri's app-data dir). |
| `events` | `EventBus<E>`, `Envelope<E>`, `Scope` | In-process broadcast of scoped events (`User(Uuid)` or `Broadcast`). The app defines the event enum and aliases the generic types. |
| `tasks` | `Task` trait, `TaskProducer`, `push`, `run_monitor` | Apalis background jobs on SQLite. See [Background Jobs](rust-background-jobs). Implies `database`. |
| `http` | `http::health::router()` | `/_status/{livez,readyz,version}`. Apps build their own `HttpServer` service around it. Implies `database`. |
| `logging` | `logging::init(level)` | Compact stdout tracing subscriber plus a panic hook. |
| (always) | `Version` | Build metadata, filled from the app's `build.rs`. |

What stays in the application, on purpose: **config** (env parsing), **state** (`AppState` with the pool, the event bus, a `TaskProducer`), **models**, **handlers**, **templates**, **tasks**, and **migrations**. Migrations in particular are the app's because there is one `_sqlx_migrations` table per database, so each app ships a single `sqlx::migrate!()` directory holding both its own migrations and the Apalis ones it needs, and hands that migrator to `Database::connect`.

Consumers today:

| Consumer | Features | Why |
|----------|----------|-----|
| `web/rust/crates/app` | all | The site. |
| `mobile/tauri/example/server` | `runtime`, `database`, `tasks`, `http`, `logging` | No SSE, so no `events`. |
| `mobile/tauri/std/rs` | `database` only | Runs on the phone; must not link axum or apalis. |

**Starting a third Rust service:** depend on the crate by path (`generic-rust-core = { path = "../../web/rust/crates/core" }` or through a workspace dependency, as `mobile/tauri/Cargo.toml` does), pick features, then write `config.rs`, `state.rs`, a `migrations/` dir, an `HttpServer` that merges `health::router()`, and a `main.rs` that spawns services on a `ShutdownHandle`. `mobile/tauri/example/server` is the smallest complete instance (about 700 lines including the scheduler and the Askama page). Its Dockerfile uses the repo root as build context so the path dependency resolves; copy that too.

**Changing core:** anything added there must make sense for all three consumers and be feature-gated if it pulls a dependency. Apalis is pinned to `1.0.0-rc.7` / `apalis-sqlite 1.0.0-rc.6` in both workspaces because 1.0.0 stable changed the API; bump them together.

## Stack

- **Framework:** Axum
- **Database:** SQLite (file-based, persistent volume)
- **Templates:** Askama (Jinja2-style, compiled into the binary)
- **Interactivity:** HTMX
- **Background jobs:** Apalis
- **Auth:** Google OAuth with session-based cookies
- **Port:** 3000

## Local development

```bash
cd web/rust
make install
make dev
```

## Deployment

```bash
make kamal ARGS="rust deploy"
```

The Kamal config builds from `web/rust/Dockerfile` with the build context set to `web/`.

## Environment variables

Defined in `config/deploy/rust.yml`:

| Variable | Value |
|----------|-------|
| `SQLITE_DATABASE_URL` | `sqlite:///data/app.db` |
| `LISTEN_ADDRESS` | `0.0.0.0:3000` |
| `HOST_NAME` | `https://rust.<zone>` |
| `LOG_LEVEL` | `info` |

Secrets (`GOOGLE_O_AUTH_CLIENT_ID`, `GOOGLE_O_AUTH_CLIENT_SECRET`) are injected from `.kamal/secrets`.

## Volumes

A persistent volume is mounted at `/data` inside the container for the SQLite database:

```yaml
volumes:
  - /var/lib/kamal-volumes/<name>-rust/data:/data
```

## Health check

Kamal checks `/_status/livez` for readiness.
