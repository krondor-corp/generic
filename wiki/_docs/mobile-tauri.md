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
let client = Client::new(&base_url)?;
let page = client.call(EntriesRequest { since: Some(cursor), limit: None }).await?;
```

The server is a free public API, so `Client::new` is all the app uses; if a server needs auth it is a property of the client (`with_bearer`), not the request, so request types stay auth-agnostic. One file per endpoint; the doc comment on each request names the server route it mirrors. The server compiles the same crate, so wire types cannot drift — only paths can, and those are documented side by side.

### `mobile-std` (Rust, `std/rs`)

What every app's `src-tauri` would otherwise reimplement:

| Module | What it does |
|--------|--------------|
| `db` | Opens `app.db` under Tauri's app-data dir and runs the app's `sqlx::migrate!()`. Uses `generic-rust-core` with only the `database` feature, so the phone build never links axum or apalis. |
| `settings` | A `kv` table for small mutable state: server URL, cursors. Created outside migrations so every app gets it for free. |
| `api` | `ServerConfig::load/save/seed_default` and `client(&settings)` → a `mobile_api::Client` for the saved server URL. |
| `poller` | A named foreground loop: run `f(app)` every `interval`, now on `trigger()`, until `stop()`. |
| `identity` | A per-install `device_id` minted on first launch, plus a display name and `Platform`. Unused by the example; there for a server that needs to tell installs apart. |
| `notify` | Local notifications via `tauri-plugin-notification`, with permission prompting. No-ops if denied. Unused by the example. |
| `Error` | One error enum, `Serialize`d as its display string so commands can return it directly. |

### `@mobile/std` (TypeScript, `std/ts`)

Deliberately tiny:

```ts
const listEntries = command<{ limit: number }, Entry[]>("list_entries");
useTauriEvent<SyncReport>("sync:done", (report) => refresh());
```

`command` names a `#[tauri::command]` once and returns a typed async function; `useTauriEvent` subscribes for a component's lifetime. If you find yourself wanting more here, the logic probably belongs in Rust.

## App layout

```
example/app/
  src/                     # React: App.tsx (one screen), api.ts (every command named once), styles.css
  public/icon.svg          # the brand mark, same file as web/rust
  app-icon.png             # source for every platform icon (make icons)
  src-tauri/
    src/
      lib.rs               # tauri::Builder: setup (open DB, spawn poller), commands
      state.rs             # AppState { db, settings }, DEFAULT_SERVER_URL, settings keys
      sync.rs              # the sync pass: page from cursor → upsert → emit
      database/            # MIGRATOR + models (device-side schema)
      commands/            # #[tauri::command]s grouped by resource
    migrations/            # this app's device schema
    capabilities/          # what the webview may call (core only)
    tauri.conf.json        # productName "Generic", identifier org.krondor.generic.ios
```

Wiring in `setup`:

```rust
let state = tauri::async_runtime::block_on(AppState::init(&handle))?;
app.manage(state);
let poller = Poller::spawn(handle, "sync", Duration::from_secs(60), sync::run);
app.manage(poller);
```

## Patterns

**Sync, not fetch.** The app never renders straight from the network. `sync::run` pulls pages past the stored cursor, `INSERT OR IGNORE`s them, advances the cursor, and the UI reads from SQLite. This gives offline reads, an honest "last synced" timestamp, and a reliable definition of "new" (a row was inserted).

**Cursor = server sequence.** The server assigns a monotonic `seq` per row; the device stores the max it has seen. "Since my cursor" is exact and idempotent, unlike timestamps.

**No setup screen.** `DEFAULT_SERVER_URL` is seeded into the kv table on first launch: debug builds point at the local server (`localhost:9100`, or `10.0.2.2` on the Android emulator), release builds at the deployed domain. The switch is `cfg!(debug_assertions)`, not an env var, because env vars do not survive Xcode's build-script phase.

**Commands return `mobile_std::Result<T>`.** The frontend sees the error's display string. Keep user-facing wording in the Rust error, not in the component.

**If you add notifications**, key them on the count of inserted rows and skip the run that had no cursor (a first sync is a backfill, not news). `tauri-plugin-notification` schedules on the device, which works any time on desktop and while the app is running on mobile. iOS and Android suspend background processes, so the poller is a foreground guarantee only; waking a sleeping phone needs server push (APNs/FCM), a different pattern.

## Platforms

| Target | Needs | First time |
|--------|-------|------------|
| Desktop (macOS) | Xcode CLT | nothing — `make dev` |
| iOS | Xcode + a simulator runtime, `aarch64-apple-ios-sim` target | `make init-ios` generates `src-tauri/gen/apple` |
| Android | Android Studio SDK + NDK, `ANDROID_HOME`/`NDK_HOME` | `make init-android` generates `src-tauri/gen/android` |

`src-tauri/gen/` is gitignored; regenerate it per machine (and after changing `identifier` or `productName`). Icons are committed and regenerated from `app-icon.png` with `make icons`; that file is the brand mark from `web/rust/crates/app/static/icon.svg` on white. `tauri ios init` writes Tauri's default icons into the generated asset catalog, so `make init-ios` runs `make icons` afterwards; do the same if you run the Tauri CLI by hand.

**Disk.** A debug iOS build on top of desktop builds puts `target/` past 10 GB, and the iOS simulator runtime is another ~8 GB. `make du` shows what the artifacts cost; `make clean` removes Rust, JS, and Xcode DerivedData output (`clean-rust`, `clean-js`, `clean-xcode` individually; `clean-gen` also drops the generated Xcode/Android projects).

## TestFlight

`make testflight` builds the app signed for App Store Connect and uploads it; the `TestFlight` workflow does the same from CI. The Apple account setup, the build-number rule, and how the script and workflow work are in [Deploying → Publishing the iOS app]({{ '/docs/deploying/' | relative_url }}#publishing-the-ios-app).

## Adding a second app

1. `mkdir mobile/tauri/<name>/app` and copy `example/app` as a starting point; change `identifier` and `productName` in `tauri.conf.json`.
2. Add `<name>/app/src-tauri` to `mobile/tauri/Cargo.toml` members and `<name>/app` to `pnpm-workspace.yaml`.
3. If it needs its own server, copy `example/server`, add it to the workspace, and give it a Kamal config + confit service entry (see [Server](mobile-api-server)).
4. Put anything the second app wants from the first into `std/`, not into a cross-import.
