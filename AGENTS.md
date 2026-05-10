# CLAUDE.md

## What this is

A monorepo for deploying web services to a single DigitalOcean droplet. Infrastructure (Terraform), provisioning (Ansible), deployment (Kamal), and application source code all live here. Config flows through `confit.toml`.

Wiki: https://generic.krondor.org/wiki/

## Layout

```
confit.toml              # Central config — all values, secrets, service definitions
Makefile                 # Top-level targets (infra, bootstrap, kamal, ssh, wiki)
bin/                     # Wrapper scripts
iac/                     # Terraform modules and stages
ansible/                 # Playbooks and tasks
config/deploy/           # Kamal deploy configs (ERB + confit)
.kamal/secrets           # Secret resolution at deploy time
web/
  py/                    # Python service (FastAPI + PostgreSQL + Redis + TaskIQ)
  rust/                  # Rust service (Axum + SQLite + HTMX + Apalis)
  ts/                    # TypeScript service (React + Vite + pnpm/turbo)
wiki/                    # Jekyll documentation site (GitHub Pages)
```

## Services

| Service | Stack | Port | Deploy config |
|---------|-------|------|---------------|
| py | FastAPI, PostgreSQL 17, Redis 7, TaskIQ, pydantic-ai | 8000 | `config/deploy/py.yml` |
| rust | Axum, SQLite, Askama, HTMX, Apalis | 3000 | `config/deploy/rust.yml` |
| ts-web | React, Vite, pnpm, turbo | 3000 | `config/deploy/ts-web.yml` |

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

- **Architecture**: `runtime::Service` trait with `run`/`spawn`/`shutdown`. One handler per file exporting `handler`. Views (GET) and actions (POST) in separate dirs.
- **Models**: three types per entity — full model, `ListItem`, `Patch` (via `struct-patch`). Private fields with getters.
- **Database**: SQLite via sqlx. `DbUuid` newtype. Migrations run on startup.
- **Background jobs**: `Task` trait with `WORKER_NAME`, `register`, `push`. Uses Apalis.
- **Real-time**: `EventBus` with scoped events (`User(Uuid)`, `Broadcast`). SSE delivery.
- **Templates**: Askama (Jinja2-style), compiled into the binary, declared in the same file as the handler.

## TypeScript service (`web/ts/`)

- **Monorepo**: pnpm workspaces + turborepo. Apps: `web/` (Vite React SPA), `api/` (Express).
- **Shared packages**: subpath exports via ts-core (`./core`, `./api`).
- **Build**: `pnpm build` runs turbo which builds all packages and apps.

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
