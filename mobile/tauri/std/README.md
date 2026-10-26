# Patterns

What every Tauri app in this repo does the same way, and the three packages that make it so.

## The split

```
 server (droplet, always on)                   device (Tauri, when running)
 ──────────────────────────────                ────────────────────────────────
 Scheduler  → Apalis task                      Poller (mobile-std) every N seconds
   └ do the work (here: write a row)             └ sync::run
   └ INSERT into SQLite (seq autoincrement)          ├ GET /api/v0/entries?since=<cursor>
                                                      ├ INSERT OR IGNORE into device SQLite
 GET /api/v0/entries?since=   ← mobile-api →         ├ cursor = response.cursor (kv)
                                                      └ emit "sync:done" → React re-reads SQLite
```

The server owns everything that must happen whether or not a phone is awake. The device owns a mirror and reacts locally. The webview is a view.

## `mobile-api` — the contract

- One request type per endpoint, one file per endpoint, implementing `ApiRequest` (`build_request(self, base, http) -> RequestBuilder`).
- `Client::call(req)` executes and decodes `req::Response`. Non-2xx → `ApiError::HttpStatus`. Empty 2xx body decodes as `()`.
- The server is open; `Client::new` is enough. If a server needs auth it lives on the client (`Client::with_bearer`), never on request types.
- Wire types shared by several endpoints live in the resource's `mod.rs`.
- Each request's doc comment says **which server route it mirrors**. Paths are the one thing the compiler cannot check — keep them in lockstep.

## `mobile-std` — the device runtime

Wire it in `setup`:

```rust
let db = mobile_std::db::open(&handle, &MIGRATOR).await?;        // app-data dir, migrated
let settings = Settings::new(db.clone());                        // kv table
ServerConfig::seed_default(&settings, DEFAULT_SERVER_URL).await?; // first launch only
app.manage(AppState { db, settings });
app.manage(Poller::spawn(handle, "sync", Duration::from_secs(60), sync::run));
```

Rules of thumb:

- **Apps own their schema.** `mobile_std::db::open` takes the app's `sqlx::migrate!()`. Only the `kv` table is created by the library.
- **Settings are strings.** `Settings::{get,set,delete}` plus `get_i64/set_i64` for cursors. Anything structured gets a real table.
- **Rebuild the API client per call.** `mobile_std::api::client(&settings)` reads the saved URL each time, so a change applies instantly.
- **`identity` and `notify` are there when you need them.** A per-install id for the day a server tells installs apart; local notifications that never fail a sync (denied permission logs and returns `Ok`). The example uses neither.
- **The poller is foreground-only.** Mobile OSes suspend the process; `tokio::time::interval` does not wake a phone. Server push (APNs/FCM) is the answer when that matters, and it is not in this example.
- **Return `mobile_std::Result<T>` from commands.** The frontend receives the display string.

## `@mobile/std` — the bridge

```ts
export const listEntries = command<{ limit: number }, Entry[]>("list_entries");
useTauriEvent<SyncReport>("sync:done", (r) => refresh());
```

Name every command once in the app's `src/api.ts` with its argument and result shapes mirroring the serde structs. If a helper here wants to know about entries, settings, or HTTP, stop: that belongs in Rust.

## Sync, precisely

1. `since = kv["sync.cursor"]`. Page `GET /entries?since&limit` until a short page, at most `MAX_PAGES` per run.
2. `INSERT OR IGNORE` each row; count the ones that landed. That count — not "the response was non-empty" — is what "new" means.
3. Store `response.cursor`. The cursor is the server's `seq`, monotonic and exact; never a timestamp.
4. Prune the mirror to the newest `RETAIN_ENTRIES` rows, matching the server's retention.
5. Record `sync.last_at` / clear `sync.last_error` (or set it), emit `sync:done` / `sync:error`.

`sync_now` (the button) and the poller both call `sync::run`, so the bookkeeping is identical. If you add notifications, key them on the count from step 2 and skip the run that had no cursor (a first sync is a backfill, not news).

## Adding an endpoint

1. `std/api/src/<resource>/<verb>.rs`: request + response types, `impl ApiRequest`, doc comment naming the route.
2. Server: `http/api/<resource>/<verb>.rs` exporting `handler`, add to `http/api/mod.rs` router. Reuse the wire types from `mobile-api`; convert from your model with `to_wire()`.
3. App: call it from `sync.rs` or a command. Expose a command in `commands/<resource>.rs`, name it in `src/api.ts`.
