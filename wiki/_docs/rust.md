---
title: Overview
order: 12
---

The Rust service lives at `web/rust/` and is deployed via Kamal with the config at `config/deploy/rust.yml`.

## Overview

The Rust stack is for lightweight, self-contained apps that don't need a separate frontend build or JavaScript framework. It compiles to a single binary with zero runtime dependencies. Pages are server-rendered with Askama templates and enhanced with HTMX for interactivity and SSE for real-time updates.

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
