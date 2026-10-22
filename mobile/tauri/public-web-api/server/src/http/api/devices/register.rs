//! `POST /api/v0/devices` — mirrors `mobile_api::devices::RegisterDeviceRequest`.

use axum::extract::State;
use axum::Json;
use mobile_api::devices::{Device as WireDevice, RegisterDeviceRequest};

use crate::database::models::Device;
use crate::http::error::ApiError;
use crate::state::AppState;

pub async fn handler(
    State(state): State<AppState>,
    Json(body): Json<RegisterDeviceRequest>,
) -> Result<Json<WireDevice>, ApiError> {
    if body.device_id.trim().is_empty() {
        return Err(ApiError::BadRequest("device_id is required".into()));
    }
    let name = body.name.trim();
    if name.is_empty() || name.len() > 128 {
        return Err(ApiError::BadRequest(
            "name is required and at most 128 chars".into(),
        ));
    }

    let device =
        Device::upsert(body.device_id.trim(), body.platform, name, &state.database).await?;
    tracing::info!(
        device_id = device.device_id(),
        name = device.name(),
        "device registered"
    );

    Ok(Json(device.to_wire()))
}
