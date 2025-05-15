mod db;
mod models;
mod repositories;
mod handlers;
mod errors;

use actix_web::{web, App, HttpServer};
use db::establish_connection;
use std::sync::{Arc, Mutex};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    
    // Create database pool
    let pool = db::create_pool().await;
    
    // Create repository
    let user_repository = UserRepository::new(pool);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_repository.clone()))
            .service(
                web::scope("/api/users")
                    .route("", web::post().to(handlers::create_user))
                    .route("/{id}", web::get().to(handlers::get_user))
                    // ... other routes
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}