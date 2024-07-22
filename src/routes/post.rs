use super::api_error;
use super::Inputs;
use actix_web::{web, HttpResponse, Responder, Result};
use iot_net::{device::Device, device_cache};
use serde_json::json;

/// Gets a devices id from its name
pub async fn device_id(info: web::Json<Inputs>) -> Result<HttpResponse> {
    info.validate(&["device_name"])?;

    match device_cache::get_device_id(&info.device_name.clone().unwrap()) {
        Ok(id) => Ok(HttpResponse::Ok().json(json!({
            "id": id
        }))),
        Err(e) => Err(api_error::general_error(e).into()),
    }
}

/// Adds new device to database if not notifies device why
pub async fn post_new_device(info: web::Json<Device>) -> impl Responder {
    match info.into_inner().save_as_new() {
        Ok(id) => HttpResponse::Ok().json(json!({
            "id": id
        })),
        Err(e) => api_error::general_error(e).into(),
    }
}

/// Adds an entry into a sensors entry list
pub async fn post_entry(info: web::Json<Inputs>) -> Result<HttpResponse> {
    let inputs = info.into_inner();
    inputs.validate(&["id", "sensor_name", "entry"])?; // Validating the provided arguments

    let mut device = Device::load(&inputs.id.clone().unwrap()).map_err(|e| api_error::device_not_found(e))?;
    println!("hello1");

    if let Err(e) = device.add_entry(&inputs.sensor_name.clone().unwrap(), inputs.entry.clone().unwrap()) {
        return Err(api_error::general_error(e));
    }
    println!("hello");
    Ok(HttpResponse::Ok().finish())
}
