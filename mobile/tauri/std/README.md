# Patterns

What every Tauri app in this repo does the same way, and the three packages that make it so.

## The split

```
 server (droplet, always on)                   device (Tauri, when running)
 ──────────────────────────────                ────────────────────────────────
 Scheduler  → Apalis task                      Poller (mobile-std) every N seconds
   └ scrape public API over reqwest              └ sync::run
   └ INSERT into SQLite (seq autoincrement)          ├ POST /api/v0/devices   (register/touch)
   └ EventBus.publish(StoryEvent::New)               ├ GET  /api/v0/stories?since=<cursor>
                                                      ├ INSERT OR IGNORE into device SQLite
 GET /api/v0/stories?since=   ← mobile-api →         ├ cursor = response.cursor (kv)
 GET /api/v0/events (SSE)                             ├ notify() if rows inserted (not on first sync)
                                                      └ emit "sync:done" → React re-reads SQLite
```

The server owns everything that must happen whether or not a phone is awake. The device owns a mirror and reacts locally. The webview is a view.

## `mobile-api` — the contract

- One request type per endpoint, one file per endpoint, implementing `ApiRequest` (`build_request(self, base, http) -> RequestBuilder`).
- `Client::call(req)` executes and decodes `req::Response`. Non-2xx → `ApiError::HttpStatus`. Empty 2xx body decodes as `()`.
- The server is open; `Client::new` is enough. If a server needs auth it lives on the client (`Client::with_bearer`), never on request types.
- Wire types shared by several endpoints live in the resource's `mod.rs`. Event payloads for SSE live in `events.rs`.
- Each request's doc comment says **which server route it mirrors**. Paths are the one thing the compiler cannot check — keep them in lockstep.

## `mobile-std` — the device runtime

Wire it in `setup`:

```rust
let db = mobile_std::db::open(&handle, &MIGRATOR).await?;        // app-data dir, migrated
let settings = Settings::new(db.clone());                        // kv table
let identity = Identity::load_or_create(&settings).await?;       // device_id + name
app.manage(AppState { db, settings, identity: Mutex::new(identity) });
app.manage(Poller::spawn(handle, "sync", Duration::from_secs(60), sync::run));
```

Rules of thumb:

- **Apps own their schema.** `mobile_std::db::open` takes the app's `sqlx::migrate!()`. Only the `kv` table is created by the library.
- **Settings are strings.** `Settings::{get,set,delete}` plus `get_i64/set_i64` for cursors. Anything structured gets a real table.
- **Rebuild the API client per call.** `mobile_std::api::client(&settings)` reads the saved URL each time; settings changes apply instantly.
- **`notify` never fails the sync.** Denied permission logs and returns `Ok`.
- **The poller is foreground-only.** Mobile OSes suspend the process; `tokio::time::interval` does not wake a phone. Server push (APNs/FCM) is the answer when that matters, and it is not in this example.
- **Return `mobile_std::Result<T>` from commands.** The frontend receives the display string.

## `@mobile/std` — the bridge

```ts
export const listStories = command<{ limit: number }, Story[]>("list_stories");
useTauriEvent<SyncReport>("sync:done", (r) => refresh());
```

Name every command once in the app's `src/api.ts` with its argument and result shapes mirroring the serde structs. If a helper here wants to know about stories, settings, or HTTP, stop: that belongs in Rust.

## Sync, precisely

1. Register/touch the device (cheap, idempotent, keeps the server's device list honest).
2. `since = kv["sync.cursor"]`. Page `GET /stories?since&limit` until a short page, at most `MAX_PAGES` per run.
3. `INSERT OR IGNORE` each row; count the ones that landed. That count — not "the response was non-empty" — is what "new" means.
4. Store `response.cursor`. The cursor is the server's `seq`, monotonic and exact; never a timestamp.
5. If there was a cursor before this run and rows landed, `notify("N new stories", first_title)`. No cursor means first run: backfill silently.
6. Record `sync.last_at` / clear `sync.last_error` (or set it), emit `sync:done` / `sync:error`.

`sync_now` (the button) and the poller both call `sync::run`, so the bookkeeping is identical.

## Adding an endpoint

1. `std/api/src/<resource>/<verb>.rs`: request + response types, `impl ApiRequest`, doc comment naming the route.
2. Server: `http/api/<resource>/<verb>.rs` exporting `handler`, add to `http/api/mod.rs` router. Reuse the wire types from `mobile-api`; convert from your model with `to_wire()`.
3. App: call it from `sync.rs` or a command. Expose a command in `commands/<resource>.rs`, name it in `src/api.ts`.
