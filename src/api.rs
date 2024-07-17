use actix_web::{web, HttpResponse, Responder};
use iot_net::device::{add_entry, save_device, Device, Entry};
use serde::{Deserialize, Serialize};

/*
    Defining responses and requests expected
*/
#[derive(Serialize, Deserialize)]
pub struct ID {
    pub id: String,
}

#[derive(Deserialize)]
pub struct AddEntry {
    pub device_id: String,
    pub sensor_name: String,
    pub entry: Entry,
}

/*
    Defining api entrance points
*/

/// Adds new device to database if not notifies device why
pub async fn request_new_device(info: web::Json<Device>) -> impl Responder {
    match save_device(&info) {
        Ok(id) => HttpResponse::Ok().json(ID { id }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error saving device: {}", e)),
    }
}

/// Adds a entry to sensor entries
pub async fn post_data(info: web::Json<AddEntry>) -> impl Responder {
    match add_entry(&info.device_id, &info.sensor_name, info.entry.clone()) {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error adding entry: {}", e)),
    }
}

/// Get device data
pub async fn get_device(_info: web::Json<ID>) -> impl Responder {
    HttpResponse::Ok()
}
