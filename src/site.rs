use actix_web::{Responder, HttpResponse};

pub async fn index() -> impl Responder {
    HttpResponse::Ok()
}

pub async fn tracker() -> impl Responder {
    HttpResponse::Ok()
}
