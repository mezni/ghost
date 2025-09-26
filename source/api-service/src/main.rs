mod core;

use actix_web::{App, HttpServer, web};
use core::errors::AppError;
use core::logger::Logger;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    Logger::init();
    Logger::info("Starting API server...");

    let pool = core::db::Db::create_pool();

    HttpServer::new(move || App::new().app_data(web::Data::new(pool.clone())))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}
