//! Example Tauri app on `mobile-std`.
//!
//! The server (`../../server`) scrapes a public API into its database.
//! This app mirrors that feed into on-device SQLite, polls for anything
//! past its cursor while running, and fires a local notification when
//! new stories arrive. The webview is a view: every byte of network and
//! storage happens here in Rust.

pub mod commands;
pub mod database;
pub mod state;
pub mod sync;

use std::time::Duration;

use tauri::Manager;

use crate::state::AppState;

/// How often the foreground poller asks the server for news.
const SYNC_INTERVAL: Duration = Duration::from_secs(60);

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn".into()),
        )
        .compact()
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle().clone();
            // Setup runs on the main thread before the event loop; blocking
            // here on the (fast) database open is the standard Tauri pattern.
            let state = tauri::async_runtime::block_on(AppState::init(&handle))?;
            app.manage(state);

            let poller =
                mobile_std::poller::Poller::spawn(handle, "sync", SYNC_INTERVAL, sync::run);
            app.manage(poller);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::status::get_status,
            commands::settings::save_settings,
            commands::settings::test_connection,
            commands::settings::reset_data,
            commands::stories::list_stories,
            commands::stories::sync_now,
            commands::stories::open_story,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
