use actix_web::web;

use crate::controllers;

pub fn auth_routes(cfg: &mut web::ServiceConfig){
   cfg.service(
    web::scope("/auth").
        route("/signup", web::post().to(controllers::auth::signup)).
        route("/login", web::post().to(controllers::auth::login))
   );
}