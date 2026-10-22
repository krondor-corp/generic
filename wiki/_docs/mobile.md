---
title: Overview
order: 17
---

Mobile apps live under `mobile/`, one subdirectory per framework. Today that is `mobile/tauri/`. Each framework directory holds a **standard library** (the patterns every app on that framework shares) and **example apps** built on it.

```
mobile/
  Makefile                 # fan-out: make check-tauri, make dev-tauri, ...
  tauri/
    std/
      api/                 # mobile-api   — HTTP contract + client (Rust, shared with the server)
      rs/                  # mobile-std   — on-device runtime (SQLite, poller, notifications)
      ts/                  # @mobile/std  — typed command/event bridge for the React frontend
    public-web-api/
      app/                 # the Tauri app (React + Vite frontend, src-tauri Rust backend)
      server/              # its Axum API server, deployed via Kamal as `mobile-api`
```

## The shape of a mobile app here

Every example follows the same split:

- **The server owns the always-on work.** Scraping, scheduled jobs, anything that must happen whether or not a phone is awake runs in an Axum service on the droplet, using the same `generic-rust-core` runtime as `web/rust` (background jobs, event bus, health checks).
- **The device owns a mirror.** The app keeps its own SQLite database, asks the server "anything new since my cursor?" while it is running, and reacts locally — notifications, UI updates — from the diff.
- **One contract, compiled twice.** `mobile-api` defines request types and wire types once; the server's routes and the app's client both compile it.
- **The webview is a view.** Network, storage, and notifications happen in Rust. The React frontend calls `#[tauri::command]`s and listens to events through `@mobile/std`.

## Services

| Service | Stack | Deploy config |
|---------|-------|---------------|
| mobile-api | Axum + SQLite + Apalis, on `generic-rust-core` | `config/deploy/mobile-api.yml` |

The app itself is not deployed by Kamal; it is built with the Tauri CLI for iOS, Android, and desktop. See [Tauri](mobile-tauri).

## Commands

```bash
cd mobile/tauri
make install          # cargo fetch + pnpm install
make dev-server       # API server on :9100 with API_TOKEN=dev-token
make dev              # desktop app (fastest inner loop)
make dev-ios          # iOS simulator (run make init-ios once first)
make dev-android      # Android emulator (run make init-android once first)
make check            # fmt-check, clippy, typecheck, tests
```
