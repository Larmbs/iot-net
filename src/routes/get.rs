use super::api_error;
use super::Inputs;
use actix_web::{web, HttpResponse, Result};
use iot_net::{device, device_cache};
use serde_json::json;

pub async fn get_devices() -> HttpResponse {
    match device_cache::get_device_id_map() {
        Ok(map) => HttpResponse::Ok().json(json!({
            "devices": map
        })),
        Err(e) => api_error::general_error(e).into(),
    }
}
