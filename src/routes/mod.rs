//! Module which defines all site routes including api ones

use serde::Deserialize;
use serde_json::json;
use iot_net::device::Entry;
use actix_web::Error;

pub mod post;
pub mod get;
pub mod site;

/* Common input structure */

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
