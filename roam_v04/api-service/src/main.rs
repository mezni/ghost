use actix_web::{App, HttpServer, Responder, get, web};
mod errors;
use errors::AppError;

#[get("/")]
async fn index() -> Result<impl Responder, AppError> {
    Ok("Hello, World!")
}

#[get("/fail")]
async fn fail() -> Result<impl Responder, AppError> {
    Err(AppError::InternalError)
}

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    println!("Starting server on http://localhost:8080");

    HttpServer::new(|| App::new().service(index).service(fail))
        .bind(("127.0.0.1", 8080))
        .map_err(|e| AppError::InternalError)?
        .run()
        .await
        .map_err(|e| AppError::InternalError)?;

    Ok(())
}
