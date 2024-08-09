use super::api_error;
use actix_web::HttpResponse;
use iot_net::device_cache;
use serde_json::json;

pub async fn get_devices() -> HttpResponse {
    match device_cache::get_device_id_map() {
        Ok(map) => HttpResponse::Ok().json(json!({
            "devices": map
        })),
        Err(e) => api_error::general_error(e).into(),
    }
}
