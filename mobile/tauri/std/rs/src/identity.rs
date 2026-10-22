//! Who this install is. A random UUID minted on first launch and kept in
//! settings — survives app restarts, not reinstalls — plus a display name
//! the user can change. Nothing in the example sends this anywhere; it is
//! here for the day a server wants to tell installs apart.

use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::settings::{keys, Settings};
use crate::Result;

/// The OS this build runs on, from compile-time `cfg`.
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
    pub const fn current() -> Self {
        if cfg!(target_os = "ios") {
            Self::Ios
        } else if cfg!(target_os = "android") {
            Self::Android
        } else if cfg!(target_os = "macos") {
            Self::Macos
        } else if cfg!(target_os = "windows") {
            Self::Windows
        } else if cfg!(target_os = "linux") {
            Self::Linux
        } else {
            Self::Unknown
        }
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ios => "ios",
            Self::Android => "android",
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::Unknown => "unknown",
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug)]
pub struct Identity {
    pub device_id: String,
    pub name: String,
    pub platform: Platform,
}

impl Identity {
    pub async fn load_or_create(settings: &Settings) -> Result<Self> {
        let device_id = match settings.get(keys::DEVICE_ID).await? {
            Some(id) => id,
            None => {
                let id = Uuid::new_v4().to_string();
                settings.set(keys::DEVICE_ID, &id).await?;
                tracing::info!(device_id = %id, "minted device id");
                id
            }
        };
        let name = match settings.get(keys::DEVICE_NAME).await? {
            Some(name) => name,
            None => {
                let name = default_name();
                settings.set(keys::DEVICE_NAME, &name).await?;
                name
            }
        };
        Ok(Self {
            device_id,
            name,
            platform: Platform::current(),
        })
    }

    pub async fn rename(&mut self, settings: &Settings, name: &str) -> Result<()> {
        settings.set(keys::DEVICE_NAME, name).await?;
        self.name = name.to_string();
        Ok(())
    }
}

fn default_name() -> String {
    let platform = match Platform::current() {
        Platform::Ios => "iPhone",
        Platform::Android => "Android",
        Platform::Macos => "Mac",
        Platform::Windows => "Windows PC",
        Platform::Linux => "Linux",
        Platform::Unknown => "Device",
    };
    platform.to_string()
}

#[cfg(test)]
mod tests {
    use super::Platform;

    #[test]
    fn platform_round_trips_through_serde() {
        let json = serde_json::to_string(&Platform::Ios).unwrap();
        assert_eq!(json, "\"ios\"");
        assert_eq!(
            serde_json::from_str::<Platform>(&json).unwrap(),
            Platform::Ios
        );
    }
}
