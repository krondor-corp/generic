---
title: Tauri
order: 18
---

[Tauri 2](https://v2.tauri.app/) builds one codebase for iOS, Android, macOS, Windows, and Linux: a Rust process hosting the platform webview. The frontend is React + Vite; everything that touches the network, disk, or OS is Rust.

Pinned to Tauri 2 stable. crates.io's "latest" `tauri` is a 3.0 alpha — the workspace manifest keeps `tauri = "2"`.

## The standard library

Three packages under `mobile/tauri/std/`:

### `mobile-api` (Rust, `std/api`)

The HTTP contract. Each endpoint is a request type that implements `ApiRequest` — it builds its own `reqwest::RequestBuilder` (method, path, query, body) — and a generic `Client` executes it and decodes the typed response:

```rust
let client = Client::with_bearer(&base_url, &token)?;
let page = client.call(StoriesRequest { since: Some(cursor), limit: None }).await?;
```

Auth is a property of the client (`with_bearer`), not the request, so request types stay auth-agnostic. One file per endpoint; the doc comment on each request names the server route it mirrors. The server compiles the same crate, so wire types cannot drift — only paths can, and those are documented side by side.

### `mobile-std` (Rust, `std/rs`)

What every app's `src-tauri` would otherwise reimplement:

| Module | What it does |
|--------|--------------|
| `db` | Opens `app.db` under Tauri's app-data dir and runs the app's `sqlx::migrate!()`. Uses `generic-rust-core` with only the `database` feature, so the phone build never links axum or apalis. |
| `settings` | A `kv` table for small mutable state: server URL, token, cursors. Created outside migrations so every app gets it for free. |
| `identity` | A per-install `device_id` minted on first launch, plus a display name. |
| `api` | `ServerConfig::load/save` and `client(&settings)` → a bearer-authed `mobile_api::Client`. |
| `notify` | Local notifications via `tauri-plugin-notification`, with permission prompting. No-ops if denied. |
| `poller` | A named foreground loop: run `f(app)` every `interval`, now on `trigger()`, until `stop()`. |
| `Error` | One error enum, `Serialize`d as its display string so commands can return it directly. |

### `@mobile/std` (TypeScript, `std/ts`)

Deliberately tiny:

```ts
const listStories = command<{ limit: number }, Story[]>("list_stories");
useTauriEvent<SyncReport>("sync:done", (report) => refresh());
```

`command` names a `#[tauri::command]` once and returns a typed async function; `useTauriEvent` subscribes for a component's lifetime. If you find yourself wanting more here, the logic probably belongs in Rust.

## App layout

```
public-web-api/app/
  src/                     # React: App.tsx, api.ts (every command named once), styles.css
  src-tauri/
    src/
      lib.rs               # tauri::Builder: plugins, setup (open DB, spawn poller), commands
      state.rs             # AppState { db, settings, identity } + app-specific settings keys
      sync.rs              # the sync pass: register → page from cursor → upsert → notify → emit
      database/            # MIGRATOR + models (device-side schema)
      commands/            # #[tauri::command]s grouped by resource
    migrations/            # this app's device schema
    capabilities/          # what the webview may call (core + notification + opener)
    tauri.conf.json
```

Wiring in `setup`:

```rust
let state = tauri::async_runtime::block_on(AppState::init(&handle))?;
app.manage(state);
let poller = Poller::spawn(handle, "sync", Duration::from_secs(60), sync::run);
app.manage(poller);
```

## Patterns

**Sync, not fetch.** The app never renders straight from the network. `sync::run` pulls pages past the stored cursor, `INSERT OR IGNORE`s them, advances the cursor, and the UI reads from SQLite. This gives offline reads, an honest "last synced" timestamp, and a reliable definition of "new" (a row was inserted) for notifications.

**Cursor = server sequence.** The server assigns a monotonic `seq` per row; the device stores the max it has seen. "Since my cursor" is exact and idempotent, unlike timestamps.

**First sync is a backfill.** No cursor yet means the user has not seen anything; sync silently, do not notify.

**Local notifications are foreground notifications.** `tauri-plugin-notification` schedules on the device, which works any time on desktop and while the app is running on mobile. iOS and Android suspend background processes, so the poller is a foreground guarantee only. Waking a sleeping phone needs server push (APNs/FCM) — a different pattern, not part of this example.

**Commands return `mobile_std::Result<T>`.** The frontend sees the error's display string. Keep user-facing wording in the Rust error, not in the component.

**Open links from Rust.** `open_story` calls the opener plugin server-side, so the webview needs no extra npm package and the capability stays in `capabilities/default.json`.

## Platforms

| Target | Needs | First time |
|--------|-------|------------|
| Desktop (macOS) | Xcode CLT | nothing — `make dev` |
| iOS | Xcode + a simulator runtime, `aarch64-apple-ios-sim` target | `make init-ios` generates `src-tauri/gen/apple` |
| Android | Android Studio SDK + NDK, `ANDROID_HOME`/`NDK_HOME` | `make init-android` generates `src-tauri/gen/android` |

`src-tauri/gen/` is gitignored; regenerate it per machine. Icons are committed and regenerated from `app-icon.png` with `make icons`.

## Adding a second app

1. `mkdir mobile/tauri/<name>/app` and copy `public-web-api/app` as a starting point; change `identifier` and `productName` in `tauri.conf.json`.
2. Add `<name>/app/src-tauri` to `mobile/tauri/Cargo.toml` members and `<name>/app` to `pnpm-workspace.yaml`.
3. If it needs its own server, copy `public-web-api/server`, add it to the workspace, and give it a Kamal config + confit service entry (see [Mobile API server](mobile-api-server)).
4. Put anything the second app wants from the first into `std/`, not into a cross-import.
