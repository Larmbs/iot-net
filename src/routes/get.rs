use super::api_error;
use super::Inputs;
use actix_web::{web, HttpResponse, Responder};
use iot_net::device_cache;
use serde_json::json;

// struct DeviceInfo {
//     name: String,
//     description: String,
//     sensors: Vec<String>,
//     config: std::collections::HashMap<String, String>,
// }

pub async fn get_devices() -> HttpResponse {
    match device_cache::get_device_id_map() {
        Ok(map) => HttpResponse::Ok().json(json!({
            "devices": map
        })),
        Err(e) => api_error::general_error(e).into(),
    }
}

/// Get basic device data
pub async fn get_device(_info: web::Json<Inputs>) -> impl Responder {
    HttpResponse::Ok()
}

// #[derive(Serialize)]
// struct SensorInfo {
//     pub name: String,
//     pub description: String,
//     pub entry_type: EntryType,
//     pub entries: usize,
// }

/// Gets specific data on a sensor
pub async fn get_device_sensor(_info: web::Json<Inputs>) -> impl Responder {
    HttpResponse::Ok()
}

/// Gets the past x device entries
pub async fn get_device_sensor_entries(_info: web::Json<Inputs>) -> impl Responder {
    HttpResponse::Ok()
}
