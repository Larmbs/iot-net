//! Module which defines API entry points

use actix_web::{web, Error, HttpResponse, Responder, Result};
use iot_net::device::{add_entry, save_device, Device, Entry, EntryType};
use serde::{Deserialize, Serialize};
use serde_json::json;

/*
    Defining responses and requests expected
*/
#[derive(Serialize, Deserialize)]
pub struct ID {
    pub id: String,
}

#[derive(Deserialize)]
pub struct Inputs {
    pub id: Option<String>,
    pub device_name: Option<String>,
    pub sensor_name: Option<String>,
    pub entry: Option<Entry>,
    pub from_time: Option<String>,
}
impl Inputs {
    fn validate(&self, required_fields: &[&str]) -> Result<(), Error> {
        for &field in required_fields {
            match field {
                "id" if self.id.is_none() => {
                    return Err(actix_web::error::ErrorBadRequest(
                        json!({"error": "id is required"}),
                    ));
                }
                "device_name" if self.device_name.is_none() => {
                    return Err(actix_web::error::ErrorBadRequest(
                        json!({"error": "device_name is required"}),
                    ));
                }
                "sensor_name" if self.sensor_name.is_none() => {
                    return Err(actix_web::error::ErrorBadRequest(
                        json!({"error": "sensor_name is required"}),
                    ));
                }
                "entry" if self.entry.is_none() => {
                    return Err(actix_web::error::ErrorBadRequest(
                        json!({"error": "entry is required"}),
                    ));
                }
                "from_time" if self.from_time.is_none() => {
                    return Err(actix_web::error::ErrorBadRequest(
                        json!({"error": "from_time is required"}),
                    ));
                }
                _ => (),
            }
        }
        Ok(())
    }
}

/*
    Defining api entrance points
*/

/// Adds new device to database if not notifies device why
pub async fn post_new_device(info: web::Json<Device>) -> impl Responder {
    match save_device(&info) {
        Ok(id) => HttpResponse::Ok().json(ID { id }),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error saving device: {}", e)),
    }
}

/// Adds an entry into a sensors entry list
pub async fn post_data(info: web::Json<Inputs>) -> Result<HttpResponse> {
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

struct DeviceInfo {
    name: String,
    description: String,
    sensors: Vec<String>,
    config: std::collections::HashMap<String, String>,
}
/// Get basic device data
pub async fn get_device(_info: web::Json<Inputs>) -> impl Responder {
    HttpResponse::Ok()
}

#[derive(Serialize)]
struct SensorInfo {
    pub name: String,
    pub description: String,
    pub entry_type: EntryType,
    pub entries: usize,
}

/// Gets specific data on a sensor
pub async fn get_device_sensor(_info: web::Json<Inputs>) -> impl Responder {
    HttpResponse::Ok()
}

/// Gets the past x device entries
pub async fn get_device_sensor_entries(_info: web::Json<Inputs>) -> impl Responder {
    HttpResponse::Ok()
}
