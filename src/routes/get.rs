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

/// Gets basic device data
pub async fn get_device(info: web::Json<Inputs>) -> Result<HttpResponse> {
    info.validate(&["id"])?;
    println!("{}", &info.id.clone().unwrap());
    let device = device::Device::load(&info.id.clone().unwrap()).map_err(api_error::device_not_found)?;
    Ok(HttpResponse::Ok().json(json!({
        "name": device.name,
        "description": device.description,
        "sensors": device.get_sensor_names(),
        "config": device.config
    })))
}

/// Gets specific data on a sensor
pub async fn get_device_sensor(info: web::Json<Inputs>) -> Result<HttpResponse> {
    info.validate(&vec!["id", "sensor_name"])?;
    let device = device::Device::load(&info.id.clone().unwrap()).map_err(api_error::device_not_found)?;
    let sensor = device.get_sensor_by_name(&info.sensor_name.clone().unwrap()).unwrap();
    Ok(HttpResponse::Ok().json(json!({
        "description": sensor.description,
        "entry_type": sensor.entry_type,
        "entry_count": sensor.entries.len()
    })))
}

/// Gets the past x device entries
pub async fn get_device_sensor_entries(info: web::Json<Inputs>) -> Result<HttpResponse> {
    info.validate(&vec!["id", "sensor_name", "from_time"])?;
    let device = device::Device::load(&info.id.clone().unwrap()).map_err(api_error::device_not_found)?;
    let sensor = device.get_sensor_by_name(&info.sensor_name.clone().unwrap()).unwrap();
    Ok(HttpResponse::Ok().json(json!({
        "entries": sensor.entries,
    })))
}
