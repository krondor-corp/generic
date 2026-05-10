---
title: Local Development
order: 15
---

Each service stack under `web/` can be developed locally with hot-reload. The Python and Rust stacks rely on containerized infrastructure (PostgreSQL, Redis, SQLite), while the TypeScript stack is self-contained.

## Prerequisites

All stacks need:

| Tool | Purpose |
|------|---------|
| `confit` | Config resolution and secret injection |
| `make` | Uniform task interface across stacks |
| `pnpm` | Node package management (Python hybrid + TypeScript) |
| `docker` / `podman` | Local infrastructure containers |

Stack-specific:

| Stack | Additional tools |
|-------|-----------------|
| Python | `uv`, `turbo` |
| Rust | `cargo`, `cargo-watch` |
| TypeScript | `turbo` |

## Starting services

### Python (FastAPI)

The Python stack needs PostgreSQL and Redis running locally. Infrastructure is managed via helper scripts in `web/bin/`:

```bash
cd web/py

# Install Python + Node dependencies
make install

# Start postgres and redis containers
make setup

# Run dev server + worker + scheduler via turbo
make dev
```

`make dev` runs turbo, which starts three processes in parallel: the FastAPI dev server, the taskiq worker, and the taskiq scheduler. Each uses `web/py/apps/py-app/bin/env.sh` to configure environment variables, start the database, and run migrations automatically.

To run only the web server without the worker and scheduler:

```bash
make dev-server
```

### Rust (Axum)

The Rust stack uses SQLite, so no external containers are needed:

```bash
cd web/rust

# Fetch dependencies
make install

# Run dev server with cargo-watch hot reload
make dev
```

The dev server creates its SQLite database at `data/app.db` automatically. Reset it with `make db-reset`.

### TypeScript (React + Express)

The TypeScript stack is fully self-contained:

```bash
cd web/ts

# Install pnpm dependencies
make install

# Run all apps in dev mode via turbo
make dev
```

Turbo starts both the Vite React SPA (`apps/web/`) and the Express API (`apps/api/`) in parallel with hot module replacement.

## Environment variables

Local dev environment is configured through two mechanisms:

1. **confit** -- the `confit.toml` at the repo root defines all credentials and service config. Dev scripts use `confit --set stage=development run --upper credentials.app` to inject secrets as environment variables.

2. **env.sh scripts** -- each app has a `bin/env.sh` that sources confit and exports the variables its process needs (`POSTGRES_URL`, `REDIS_URL`, `HOST_NAME`, `LISTEN_PORT`, etc.).

The env.sh pattern for the Python stack also handles worktree-aware configuration. When working on a feature branch, `confit resolve worktree.*` values adjust the database name and port offset so multiple branches can run simultaneously.

## Database management

### Python (PostgreSQL + Alembic)

```bash
cd web/py

# Run pending migrations
make db-migrate

# Create a new migration
make db-prepare MSG="add users table"

# Drop and recreate the database
make wipe
make setup
```

### Rust (SQLite)

```bash
cd web/rust

# Delete the database (migrations run on startup)
make db-clean

# Full reset
make db-reset
```

## Hot reload

Each stack handles hot reload differently:

- **Python**: `uvicorn --reload` watches Python source files and restarts the server on changes.
- **Rust**: `cargo-watch` rebuilds and restarts the binary when Rust sources change.
- **TypeScript**: Vite's HMR provides instant updates in the browser. The Express API uses its own dev watcher.

## Makefile conventions

Every stack exposes the same core set of make targets:

```
make install     # Install dependencies
make dev         # Run in development mode with hot reload
make build       # Build for production
make test        # Run tests
make lint        # Run linter
make fmt         # Format code
make fmt-check   # Check formatting without modifying
make types       # Type checking
make check       # Run all checks (fmt-check + lint + types + test)
make clean       # Remove build artifacts
```

This uniformity means you can `cd` into any stack and use the same commands without remembering stack-specific tooling.
