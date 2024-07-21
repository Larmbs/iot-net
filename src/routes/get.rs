use super::api_error;
use super::Inputs;
use actix_web::{web, HttpResponse, Responder, Result};
use iot_net::{device, device_cache};
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

/// Gets basic device data
pub async fn get_device(info: web::Json<Inputs>) -> Result<HttpResponse> {
    info.validate(&["id"])?;
    let device = device::Device::load(&info.id.clone().unwrap()).map_err(api_error::device_not_found)?;
    Ok(HttpResponse::Ok().json(json!({
        "name": device.name,
        "description": device.description,
        "sensors": device.get_sensor_names(),
        "config": device.config
    })))
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
