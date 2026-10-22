# Tauri

Mobile (and desktop) apps on [Tauri 2](https://v2.tauri.app/): a Rust process hosting the platform webview, React + Vite in front. One codebase for iOS, Android, macOS, Windows, Linux.

Full docs: https://generic.krondor.org/wiki/ → Mobile.

## Layout

```
std/
  api/           mobile-api    HTTP contract + client. Compiled by the app AND the server.
  rs/            mobile-std    On-device runtime: SQLite, settings, identity, notify, poller.
  ts/            @mobile/std   Typed command()/useTauriEvent() bridge for React.
  README.md                    The patterns.
public-web-api/
  app/           The example Tauri app (src/ React, src-tauri/ Rust).
  server/        Its Axum server: marketing page + device API — deployed via Kamal as `ios`.
  README.md
```

Two workspaces, one directory: `Cargo.toml` (Rust) and `pnpm-workspace.yaml` (TS). Shared plumbing comes from `web/rust/crates/core` by path.

## Quick start

```bash
make install        # cargo fetch + pnpm install
make dev-server     # API server on :9100, no auth
make dev            # desktop app, pointed at http://localhost:9100 (override in Settings)
make check          # fmt-check, clippy, typecheck, tests
```

iOS and Android need their generated projects once per machine: `make init-ios` / `make init-android`, then `make dev-ios` / `make dev-android`.

## Versions

- Tauri **2** stable (`tauri = "2"`). crates.io's latest `tauri` is a 3.0 alpha; do not float.
- Apalis pinned to the same rc versions as `web/rust` (`=1.0.0-rc.7` / `=1.0.0-rc.6`); 1.0.0 stable changed the API.
