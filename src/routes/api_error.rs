//! Module which defines common api errors
use actix_web::Error;
use anyhow;

/// Error thrown when device name or device id provided not found in database
#[inline]
pub fn device_not_found(e: anyhow::Error) -> Error {
    actix_web::error::ErrorNotFound(format!("Device not found: {e}"))
}

/// General conversion between anyhow errors and internal server errors
#[inline]
pub fn general_error(e: anyhow::Error) -> Error {
    actix_web::error::ErrorInternalServerError(e)
}
