use iot_net::{device::{Device, save_device, add_entry, Entry}, config::load_config};
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
 
/// Adds new device to database if not notifies device why
async fn request_new_device(info: web::Json<Device>) -> impl Responder {
    match save_device(&info) {
        Ok(id) => HttpResponse::Ok().json(ID{id}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error saving device: {}", e)),
    }
}

#[derive(Serialize)]
struct ID {
    pub id: String,
}

#[derive(Deserialize)]
struct AddEntry {
    pub device_id: String,
    pub sensor_name: String,
    pub entry: Entry,
}
/// Adds a entry to sensor entries
async fn post_data(info: web::Json<AddEntry>) -> impl Responder {
    match add_entry( &info.device_id, &info.sensor_name, info.entry.clone()) {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error adding entry: {}", e)),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = load_config("./data/config.json").expect("Failed to load server config files");

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "Hello, World!" }))
            .route("/devices/new", web::post().to(request_new_device))
            .route("/devices/post", web::post().to(post_data))
    })
    .bind(config.get_socket_addr().unwrap())?
    .run()
    .await
}
