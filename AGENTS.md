# CLAUDE.md

## What this is

A monorepo for deploying web services to a single DigitalOcean droplet, plus mobile apps that talk to them. Infrastructure (Terraform), provisioning (Ansible), deployment (Kamal), and application source code all live here. Config flows through `confit.toml`.

Wiki: https://generic.krondor.org/wiki/

## Layout

```
confit.toml              # Central config — all values, secrets, service definitions
Makefile                 # Top-level targets (infra, bootstrap, kamal, ssh, wiki)
bin/                     # Wrapper scripts (kamal, ssh, testflight, ...)
iac/                     # Terraform modules and stages
ansible/                 # Playbooks and tasks
config/deploy/           # Kamal deploy configs (ERB + confit)
.kamal/secrets           # Secret resolution at deploy time
web/
  py/                    # Python service (FastAPI + PostgreSQL + Redis + TaskIQ)
  rust/                  # Rust service (Axum + SQLite + HTMX + Apalis); crates/core is shared runtime
  ts/                    # TypeScript service (React + Vite + pnpm/turbo)
mobile/
  tauri/                 # Tauri 2 apps: std/ (api, rs, ts libs) + example/ (app + server)
wiki/                    # Jekyll documentation site (GitHub Pages)
```

## Services

| Service | Stack | Port | Deploy config |
|---------|-------|------|---------------|
| py | FastAPI, PostgreSQL 17, Redis 7, TaskIQ, pydantic-ai | 8000 | `config/deploy/py.yml` |
| rust | Axum, SQLite, Askama, HTMX, Apalis | 3000 | `config/deploy/rust.yml` |
| ts-web | React, Vite, pnpm, turbo | 3000 | `config/deploy/ts-web.yml` |
| ios | Axum, SQLite, Apalis, Askama (on `generic-rust-core`): marketing page + device API | 3000 | `config/deploy/ios.yml` |

## Common commands

```bash
# Infrastructure
make infra ARGS="plan"       # Plan terraform changes
make infra ARGS="apply"      # Apply terraform changes
make bootstrap               # Ansible: provision the server
make kamal ARGS="py deploy"  # Deploy a service
make ssh USER=admin          # SSH into the server
make validate                # Check all confit values resolve
make wiki                    # Serve wiki locally
make clean                   # Remove build artifacts everywhere (clean-web, clean-mobile, clean-wiki)
make du                      # Show what build artifacts cost on disk

# Per-service (cd into web/py, web/rust, or web/ts)
make install                 # Install dependencies
make dev                     # Run dev server with hot reload
make check                   # Run all checks (fmt, lint, types, test)
make build                   # Build for production
make clean                   # Remove build artifacts
```

## Config resolution

Everything flows through `confit.toml`. Providers:
- `op://` — 1Password secrets
- `tf://` — Terraform outputs (server IP, SSH keys)
- `secret://` — marks values as sensitive
- `{ref}` — interpolation between config values

Kamal configs are ERB templates that shell out to `confit resolve`. Ansible playbooks use `lookup('pipe', 'confit ...')`.

## Python service (`web/py/`)

- **Architecture**: functional-first. Models are data containers. Business logic is `async (params, ctx) -> result`. Context dataclass for DI (db, logger, storage).
- **Packages**: `py-core` (shared library — database, AI, events) and `py-app` (FastAPI application).
- **Database**: PostgreSQL + SQLAlchemy async + Alembic. Soft deletes only (`archived_at`). uuid7 for IDs. Use `make db-prepare MSG="..."` to create migrations.
- **Background jobs**: TaskIQ + Redis broker. `@broker.task` for one-off, `@cron` for scheduled with distributed locking.
- **AI**: pydantic-ai agents with `AgentSpec` (model, system prompt, tools). Sync tools return `completed()`, async tools return `pending()` and dispatch a TaskIQ job. SSE streaming via Redis pubsub.
- **Events**: `EventPublisher` writes to per-user Redis channels. SSE endpoints subscribe and deliver to HTMX.

### StringEnum gotcha

When filtering in SQLAlchemy, use `.value` with Core-style queries but not with ORM-style:
```python
# Core (select): use .value
select(Model).where(Model.status == MyEnum.ACTIVE.value)
# ORM (query): don't use .value
session.query(Model).filter(Model.status == MyEnum.ACTIVE)
```

## Rust service (`web/rust/`)

- **Crates**: `crates/core` (`generic-rust-core`, feature-gated: runtime, database, events, tasks, http, logging) and `crates/app` (the site). Shared plumbing goes in core; config, models, handlers, tasks, and migrations stay in the app. The mobile API server depends on core by path.
- **Architecture**: `runtime::Service` trait with `run`/`spawn`/`shutdown`. One handler per file exporting `handler`. Views (GET) and actions (POST) in separate dirs.
- **Models**: three types per entity — full model, `ListItem`, `Patch` (via `struct-patch`). Private fields with getters.
- **Database**: SQLite via sqlx. `DbUuid` newtype. Migrations run on startup.
- **Background jobs**: `Task` trait (core) with `type Context`, `WORKER_NAME`, `register`; enqueue via `TaskProducer::push`. Uses Apalis, pinned to `=1.0.0-rc.7`/`=1.0.0-rc.6` (1.0.0 stable changed the API). Migrations are the app's: `Database::connect(url, &MIGRATOR)`.
- **Real-time**: `EventBus` with scoped events (`User(Uuid)`, `Broadcast`). SSE delivery.
- **Templates**: Askama (Jinja2-style), compiled into the binary, declared in the same file as the handler.

## TypeScript service (`web/ts/`)

- **Monorepo**: pnpm workspaces + turborepo. Apps: `web/` (Vite React SPA), `api/` (Express).
- **Shared packages**: subpath exports via ts-core (`./core`, `./api`).
- **Build**: `pnpm build` runs turbo which builds all packages and apps.

## Mobile (`mobile/tauri/`)

- **Shape**: server owns always-on work (scheduled jobs) on `generic-rust-core`; device keeps a SQLite mirror and polls "since cursor" while running. The webview is a view — network/storage happen in Rust.
- **Std libs**: `std/api` (`mobile-api`: `ApiRequest`/`Client` contract, one file per endpoint, compiled by app and server), `std/rs` (`mobile-std`: db, settings kv, identity, api client, notify, poller, `Error`), `std/ts` (`@mobile/std`: `command()`, `useTauriEvent()` only).
- **Example**: `example/` — deliberately minimal. `server/` (Axum; Askama page at `/`, `GET /api/v0/entries` under it, Scheduler → Apalis `HeartbeatTask` → one row per interval in `entries`; open, no auth; deployed as `ios` at `ios.generic.krondor.org`) and `app/` (Tauri 2 + React, one screen, no settings; product "Generic", bundle id `org.krondor.generic.ios`; `src-tauri/src/sync.rs` is the sync pass). Replace the task body with real work; keep the shape.
- **Workspaces**: `mobile/tauri/Cargo.toml` + `pnpm-workspace.yaml`. Tauri pinned to 2 stable (3.0 alpha is "latest" on crates.io). `make dev` (desktop), `make dev-ios`, `make dev-android`, `make dev-server`.
- **Adding an endpoint**: request type in `std/api` (doc the route) → handler file in `server/src/http/api/<resource>/` → command in `app/src-tauri/src/commands/` → name it in `app/src/api.ts`.
- Device-side `src-tauri/gen/` is generated per machine (`make init-ios`/`init-android`) and gitignored. Icons come from `app-icon.png` via `make icons`.
- **TestFlight**: `bin/testflight` (root `make testflight`) builds signed for App Store Connect and uploads with `xcrun altool`; `.github/workflows/testflight.yml` runs it on push to `main`. Credentials are `credentials.apple.*` in confit (one 1Password item). Build number = UTC minute via `bundle.iOS.bundleVersion`, never Tauri's `--build-number`.

## Wiki (`wiki/`)

Jekyll site deployed to GitHub Pages. Served locally with `make wiki`. Docs live in `wiki/_docs/`, nav in `wiki/_data/nav.yml`.

## Working with this repo

- Always use `make` targets — don't run raw `terraform`, `ansible-playbook`, or `kamal` directly. The wrapper scripts in `bin/` handle SSH agent setup, secret injection, and config resolution.
- `confit.toml` is the single source of truth. Don't hardcode IPs, domains, or credentials anywhere else.
- Each service has its own Makefile with the same interface (`install`, `dev`, `check`, `build`, `clean`).
- Kamal deploy configs use ERB to call `confit resolve` — edit the YAML carefully.
- The Python service is the most feature-complete. Start there for most app work.
- Never hard delete database records in the Python service. Use soft deletes.
- Never modify existing Alembic migrations once pushed.
