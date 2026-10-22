//! `/api/v0/devices` — installs that talk to this server.

mod register;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use register::RegisterDeviceRequest;

/// Which OS an install runs on. Serialized lowercase on the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    Ios,
    Android,
    Macos,
    Windows,
    Linux,
    Unknown,
}

impl Platform {
    /// The platform this binary was compiled for.
    pub const fn current() -> Self {
        if cfg!(target_os = "ios") {
            Platform::Ios
        } else if cfg!(target_os = "android") {
            Platform::Android
        } else if cfg!(target_os = "macos") {
            Platform::Macos
        } else if cfg!(target_os = "windows") {
            Platform::Windows
        } else if cfg!(target_os = "linux") {
            Platform::Linux
        } else {
            Platform::Unknown
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    /// Stable per-install id the device generated on first launch.
    pub device_id: String,
    pub platform: Platform,
    pub name: String,
    pub last_seen_at: String,
}
