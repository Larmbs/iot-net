use super::Inputs;
use actix_web::{web, HttpResponse, Responder, Result};
use iot_net::device::Device;
use serde_json::json;
use super::api_error;

/// Adds new device to database if not notifies device why
pub async fn post_new_device(mut info: web::Json<Device>) -> impl Responder {
    match info.save_as_new() {
        Ok(id) => HttpResponse::Ok().json(json!({
            "id": id
        })),
        Err(e) => api_error::general_error(e).into(),
    }
}

/// Adds an entry into a sensors entry list
pub async fn post_entry(info: web::Json<Inputs>) -> Result<HttpResponse> {
    info.validate(&["id", "sensor_name", "entry"])?; // Validating the provided arguments

    let mut device = Device::load(&info.id.clone().unwrap()).map_err(|e| api_error::device_not_found(e))?;

    if let Err(e) = device.add_entry(info.sensor_name.as_ref().unwrap(), info.entry.clone().unwrap()) {
        return Err(api_error::general_error(e));
    }
    
    Ok(HttpResponse::Ok().finish())
}
