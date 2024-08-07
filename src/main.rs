use actix_web::{web, App, HttpServer};

mod config;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Getting server config
    let config = config::load_config("./config/server_config.json").expect("Failed to load server config files");

    let address = config.get_socket_addr().expect("Address provided is invalid");

    // Printing some server info
    println!("Starting Server...");
    println!("Listening on http://{:?}/", address);
    println!("^C to Shutdown Server:");

    HttpServer::new(|| {
        App::new()
            /* Home Page Route */
            .route("/", web::get().to(routes::site::index))
            /* API routes for devices to interact with */
            // Post Request
            .route("/api/post/new", web::post().to(routes::post::post_new_device))
            .route("/api/post/entry", web::post().to(routes::post::post_entry))
            .route("/api/post/device_id", web::post().to(routes::post::device_id))
            // Get Requests
            .route("/api/get/devices", web::get().to(routes::get::get_devices))
            .route("/api/get/device", web::post().to(routes::get::get_device))
            .route("/api/get/device/sensor", web::post().to(routes::get::get_device_sensor))
            .route("/api/get/device/sensor/entries", web::post().to(routes::get::get_device_sensor_entries))
            /* Site Routes */
            .route("/home", web::get().to(routes::site::index))
            .route("/about", web::get().to(routes::site::about))
            .service(actix_files::Files::new("/static", "static"))
    })
    .bind(address)?
    .max_connections(config.max_clients)
    .run()
    .await
}
