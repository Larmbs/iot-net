use actix_web::{web, App, HttpServer};
use iot_net::config;

mod api;
use api::*;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config =
        config::load_config("./data/config.json").expect("Failed to load server config files");

    println!("Starting Server...");
    println!("^C to Shutdown Server:");

    HttpServer::new(|| {
        App::new()
            // Home page route
            .route("/", web::get().to(|| async { "Hello, World!" }))
            // API routes for devices to interact with
            .route("/devices/new", web::post().to(request_new_device))
            .route("/devices/post", web::post().to(post_data))
            .route("/devices/device", web::post().to(get_device))
            .route("/devices/device/sensor", web::post().to(get_device))
            .route("/devices/device/sensor/entries", web::post().to(get_device))
            // Routes for site pages
            .route("/tracker", web::post().to(get_device))
    })
    .bind(config.get_socket_addr().unwrap())?
    .max_connections(config.max_clients)
    .run()
    .await
}
