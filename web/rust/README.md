# Rust App

Axum web server with SQLite, HTMX, and background task processing.

## Quick Start

```bash
cargo build
make dev          # creates data/ dir and starts the server
```

## Architecture

Two crates. `core` is the shared runtime (also used by `mobile/tauri`);
`app` is this site.

```
crates/core/                   # generic-rust-core — shared, feature-gated
├── src/
│   ├── runtime/           # Service trait + graceful shutdown
│   ├── database/          # SQLite pool (takes the app's migrator), DbUuid
│   ├── events/            # EventBus<E> (broadcast channel, scoped envelopes)
│   ├── tasks/             # Apalis Task trait, TaskProducer, run_monitor
│   ├── http/health/       # /_status/{livez,readyz,version}
│   ├── logging.rs         # tracing subscriber + panic hook
│   └── version.rs         # Version struct (values from the app's build.rs)
crates/app/                    # generic-rust-app — this site
├── src/
│   ├── main.rs           # Entry point: config, logging, service spawning
│   ├── config.rs          # Environment-based configuration
│   ├── state.rs           # AppState: shared state across services
│   ├── http/              # Axum HTTP server (Service impl)
│   │   ├── auth/          # OAuth (Google) + session extractors
│   │   │   └── google/    # Login + callback handlers
│   │   ├── html/          # HTMX page handlers
│   │   │   ├── widgets/   # Widget CRUD (views/ + actions/)
│   │   │   └── admin/     # Admin dashboard (views/ + actions/)
│   │   └── sse/           # Server-sent events
│   ├── database/          # MIGRATOR + models
│   │   └── models/        # User, Widget (struct-patch for partial updates)
│   ├── tasks/             # TaskContext + TaskWorker (Service impl)
│   │   └── widget/        # Widget-specific tasks
│   └── events/            # AppEvent enum + EventBus/Envelope aliases
├── migrations/            # SQLx migrations (app + apalis)
├── templates/             # Askama HTML templates
└── static/                # CSS, favicon
```

## Key Patterns

- **`runtime::Service`** — implement `run(state, shutdown_rx)`, get `spawn()` for free. Each service owns its shutdown logic.
- **Models** — private fields with getters. `struct-patch` generates `UserPatch`/`WidgetPatch` for partial updates via `model.patch(patch, db)`.
- **Handlers** — one file per handler, each exports `handler`. Organized under `views/` and `actions/` directories.
- **Events** — scoped (`User(Uuid)` or `Broadcast`), two-level serde tagging (`AppEvent::Widget(WidgetEvent)`). SSE filters by scope.

## Commands

```bash
cargo build                    # Build
cargo test                     # Test
cargo clippy -- -D warnings    # Lint
cargo fmt -- --check           # Format check
cargo run --bin seed           # Seed the database
```
