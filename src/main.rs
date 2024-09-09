use actix_web::{middleware, web, App, HttpServer};
use routes::{auth::auth_routes, todo::todo_routes};
use env_logger;

mod db;
mod models;
mod schema;
mod routes;
mod controllers;
mod middlewares;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let pool = db::connect::establish_connection();
    println!("Server Starting on Port 8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) 
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Trim))
            .configure(auth_routes) 
            .configure(todo_routes)  
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
