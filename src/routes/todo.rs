use actix_web::web;

use crate::controllers;

pub fn todo_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/todo").
        route("/", web::get().to(controllers::todo::get_todos))
    );
}