use actix_web::{web, Responder, HttpResponse, Result};
use iot_net::device::{save_device, add_entry, Device};
use serde_json::json;
use super::Inputs;

/// Adds new device to database if not notifies device why
pub async fn post_new_device(info: web::Json<Device>) -> impl Responder {
    match save_device(&info) {
        Ok(id) => HttpResponse::Ok().json(json!({
            "id": id
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error saving device: {}", e)),
    }
}

/// Adds an entry into a sensors entry list
pub async fn post_entry(info: web::Json<Inputs>) -> Result<HttpResponse> {
    // Validate the input
    info.validate(&["id", "sensor_name", "entry"])?;

    // Add entry
    match add_entry(
        info.id.as_ref().unwrap(),
        info.sensor_name.as_ref().unwrap(),
        info.entry.clone().unwrap(),
    ) {
        Ok(()) => Ok(HttpResponse::Ok().finish()),
        Err(e) => Err(actix_web::error::ErrorInternalServerError(format!(
            "Error adding entry: {}",
            e
        ))),
    }
}