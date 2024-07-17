use actix_web::{web, App, HttpServer};
use iot_net::config;

mod api;
use api::*;

mod site;
use site::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config =
        config::load_config("./data/config.json").expect("Failed to load server config files");

    println!("Starting Server...");
    println!("^C to Shutdown Server:");

    HttpServer::new(|| {
        App::new()
            /* Home Page Route */ 
            .route("/", web::get().to(index))

            /* API routes for devices to interact with */
            // Post Request
            .route("/devices/new", web::post().to(post_new_device))
            .route("/devices/post", web::post().to(post_data))
            // Get Requests
            .route("/devices/device", web::get().to(get_device))
            .route("/devices/device/sensor", web::get().to(get_device_sensor))
            .route("/devices/device/sensor/entries", web::get().to(get_device_sensor_entries))

            /* Site Routes */
            .route("/tracker", web::post().to(tracker))
    })
    .bind(config.get_socket_addr().unwrap())?
    .max_connections(config.max_clients)
    .run()
    .await
}
