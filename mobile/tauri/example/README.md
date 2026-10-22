# example

The smallest end-to-end mobile app that is still real: a long-running server with a database and one scheduled job, and a phone app that mirrors its rows. Everything a bigger app needs is already in place — the shared core library, the typed contract, the deploy — so start here and add the complicated parts yourself.

- **`server/`** — Axum + SQLite + Apalis on `generic-rust-core`. `Scheduler` → `HeartbeatTask` → one row in the `entries` table every `POLL_INTERVAL_SECS`; `PruneTask` every `PRUNE_INTERVAL_SECS` keeps the newest `RETAIN_ENTRIES`. Serves a page at `/` (Askama; shows the live rows and links to TestFlight once `TESTFLIGHT_URL` is set) and `GET /api/v0/entries?since=&limit=`. No auth — it is a free public API. Deployed as `ios` at `ios.generic.krondor.org` (`config/deploy/ios.yml`).
- **`app/`** — Tauri 2, bundle id `org.krondor.generic.ios`. `src-tauri/` opens the device DB with `mobile-std`, runs a 60s `Poller` calling `sync::run`, and exposes `get_status`, `list_entries`, `sync_now`. `src/` is one React screen: the rows, a Sync button, a status line.

## Run it

```bash
cd mobile/tauri
make dev-server          # terminal 1: http://localhost:9100
make dev                 # terminal 2: desktop app
```

Debug builds (`make dev`, `make dev-ios`, `make dev-android`) point at the local server; release builds at `https://ios.generic.krondor.org`. There is no settings screen. Lower `POLL_INTERVAL_SECS` on the server to watch rows arrive.

Server env and routes: `server/src/config.rs` and the wiki's *Mobile → Server*. Device-side patterns: `../std/README.md`.

## Where things are decided

| Decision | Where |
|----------|-------|
| What the job does, how often | `server/src/tasks/heartbeat.rs`, `POLL_INTERVAL_SECS` in `server/src/config.rs` |
| How much history to keep | `server/src/tasks/prune.rs`, `RETAIN_ENTRIES` / `PRUNE_INTERVAL_SECS`; `RETAIN_ENTRIES` in `app/src-tauri/src/sync.rs` for the mirror |
| Wire shapes | `../std/api/src/{entries,health}` |
| Server schema | `server/migrations/` |
| Device schema | `app/src-tauri/migrations/` |
| Sync rules | `app/src-tauri/src/sync.rs` |
| Which server the app talks to | `DEFAULT_SERVER_URL` in `app/src-tauri/src/state.rs` |
| What the webview may do | `app/src-tauri/capabilities/default.json` |
| Poll interval on device | `SYNC_INTERVAL` in `app/src-tauri/src/lib.rs` |
