//! `#[tauri::command]`s, grouped by resource. A command's name is its
//! function name; the frontend names each once in `src/api.ts`.

pub mod settings;
pub mod status;
pub mod stories;
