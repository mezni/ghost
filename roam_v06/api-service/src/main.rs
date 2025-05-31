mod errors;
mod infra;

use actix_cors::Cors;
use actix_web::{App, HttpServer, http::header};
use infra::{config::load_config, logger::Logger};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Logger::init();

    let config = load_config().unwrap_or_else(|e| {
        Logger::error(&format!("Failed to load config: {}", e));
        std::process::exit(1);
    });

    let host = config.service.host;
    let port = config.service.port as u16;
    let allowed_origins = Arc::new(config.service.cors);

    Logger::info(&format!(
        "Starting server at http://{}:{} with CORS origins: {:?}",
        host, port, allowed_origins
    ));

    HttpServer::new(move || {
        let cors = {
            let allowed_origins = allowed_origins.clone();

            Cors::default()
                .allowed_origin_fn(move |origin, _req_head| {
                    if let Ok(origin_str) = origin.to_str() {
                        allowed_origins.iter().any(|allowed| allowed == origin_str)
                    } else {
                        false
                    }
                })
                .allowed_methods(vec![
                    "GET", "POST", "PUT", "DELETE", "PATCH", "OPTIONS", "HEAD",
                ])
                .allowed_headers(vec![
                    header::AUTHORIZATION,
                    header::CONTENT_TYPE,
                    header::ACCEPT,
                    header::ORIGIN,
                    header::USER_AGENT,
                    header::REFERER,
                    header::HeaderName::from_static("x-requested-with"),
                ])
                .supports_credentials()
        };

        App::new().wrap(cors)
        // add routes here
    })
    .bind((host.as_str(), port))?
    .run()
    .await
}
