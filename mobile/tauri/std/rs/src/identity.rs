//! Who this install is. A random UUID minted on first launch and kept in
//! settings — survives app restarts, not reinstalls — plus a display name
//! the user can change.

use mobile_api::devices::{Platform, RegisterDeviceRequest};
use uuid::Uuid;

use crate::settings::{keys, Settings};
use crate::Result;

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

    /// The registration call for this install.
    pub fn register_request(&self) -> RegisterDeviceRequest {
        RegisterDeviceRequest {
            device_id: self.device_id.clone(),
            platform: self.platform,
            name: self.name.clone(),
        }
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
