---
title: Overview
order: 13
---

The TypeScript stack is for rich client-side apps. It ships a React SPA with Vite, a pnpm/turbo monorepo, and shared packages. Use it standalone or pair it with the Python backend via the [hybrid stack]({% link _docs/hybrid-stack.md %}) pattern.

The service lives at `web/ts/` and is deployed via Kamal with the config at `config/deploy/ts-web.yml`.

## Stack

- **Framework:** Next.js
- **Monorepo:** Turborepo (`web/ts/apps/web/`)
- **Port:** 3000

## Local development

```bash
cd web/ts
make install
make dev
```

## Deployment

```bash
make kamal ARGS="ts-web deploy"
```

The Kamal config builds from `web/ts/apps/web/Dockerfile` with the build context set to `web/`.

## Environment variables

No clear environment variables are configured by default. Add them to the `env.clear` section in `config/deploy/ts-web.yml` as needed.

## Health check

Kamal checks `/` for readiness.
