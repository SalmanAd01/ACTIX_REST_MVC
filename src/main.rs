use actix_web::{middleware, web, App, HttpServer};
use routes::auth::auth_routes;
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
    // Initialize the logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let pool = db::connect::establish_connection();
    println!("Server Starting on Port 8080");

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))  // Pass the database pool to the app
            .wrap(middleware::NormalizePath::new(middleware::TrailingSlash::Trim))
            .configure(auth_routes)  // Configure routes
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
