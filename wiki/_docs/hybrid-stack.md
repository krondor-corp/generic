---
title: Hybrid Stack
order: 16
---

The hybrid stack combines a Python FastAPI backend with a TypeScript Vite SPA frontend in a single turbo workspace and a single deployable container. The Python service serves the API, admin pages (Jinja2 + HTMX), and the Vite-built SPA — all from one process, one domain, one SSL cert.

## Why

Separate frontend and backend deployments mean CORS config, two containers, two health checks, two CI pipelines. The hybrid approach collapses this into one:

- Same-origin API calls, no CORS.
- One container, one deploy, one health check.
- Server-rendered admin pages coexist with the SPA.
- Static file performance is fine — FastAPI's `StaticFiles` middleware handles it.

## Workspace layout

The Python and TypeScript code live in the same turbo workspace under `web/py/`:

```
web/py/
  apps/
    py-app/              # FastAPI application
      src/               # Python source
      static/            # ← Vite output builds here
      templates/         # Jinja2 templates (admin, emails)
    ts-web-app/          # Vite React SPA source
  packages/
    py-core/             # Shared Python library (db, AI, events)
    ts-core/             # TypeScript types mirroring py-core
  turbo.json             # Orchestrates both Python and TS tasks
  pnpm-workspace.yaml    # pnpm workspace (includes ts-web-app)
  pyproject.toml         # uv workspace (Python packages)
```

Turbo orchestrates both languages. `pnpm dev` (via `make dev`) starts the FastAPI server, taskiq worker, taskiq scheduler, and the Vite dev server in parallel. On build, the Vite SPA compiles into `apps/py-app/static/` so the Python Dockerfile can bundle everything.

## Route priority

```
1. /api/*        → FastAPI API endpoints
2. /_status/*    → Health checks
3. /admin/*      → Jinja2 + HTMX server-rendered pages
4. /static/*     → Vite-built assets (JS, CSS, images)
5. /*            → SPA catch-all (serves index.html)
```

The catch-all lets React Router handle client-side paths. API routes are registered first so they always win.

## Shared types

`packages/ts-core/` mirrors the types from `py-core`. The Python models are the source of truth — `ts-core` manually defines matching TypeScript interfaces and keeps them in sync. This avoids code generation tooling and keeps both sides readable.

## Real-time

SSE is the default for real-time. The Python service publishes events to per-user Redis channels via `EventPublisher`. The SSE endpoint subscribes and delivers them to the browser. HTMX connects via `sse-connect`, the React SPA uses `EventSource`. Both work same-origin with no proxy config.

## Docker build

The Dockerfile at `web/py/Dockerfile` bundles both Python and the pre-built static assets. The Vite build must complete before the Docker image is created — turbo handles this dependency. The resulting container runs a single process serving the API, admin pages, and SPA.
