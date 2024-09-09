use actix_web::{HttpResponse, Responder};


pub async fn get_todos() -> impl Responder {
    HttpResponse::Ok().json("Get todos")
}