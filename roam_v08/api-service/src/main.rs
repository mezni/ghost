// src/main.rs

mod errors;
mod infra;
use errors::AppError;
use infra::logger::Logger;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    Logger::init();

    Logger::info("Application starting up...");

    let server_config = match infra::config::load_config() {
        Ok(config) => {
            Logger::info("Application configuration loaded successfully.");
            config
        }
        Err(e) => {
            Logger::error(&format!("Failed to load configuration: {}", e));
            match e {
                AppError::ConfigError(config_err) => {
                    Logger::error(&format!("Detailed Config Error: {}", config_err));
                }
                AppError::MissingEnvVar(var_name) => {
                    Logger::error(&format!("Missing environment variable: {}", var_name));
                }
                _ => { /* Handle other AppError variants if needed */ }
            }
            std::process::exit(1);
        }
    };

    // Initialize the database pool
    let _db_pool: PgPool = match infra::postgres::db::init_db_pool(&server_config.database).await {
        // Await the async function
        Ok(pool) => {
            Logger::info("Database connection pool initialized successfully.");
            pool
        }
        Err(e) => {
            Logger::error(&format!("Failed to initialize database pool: {}", e));
            std::process::exit(1);
        }
    };

    Logger::info(&format!(
        "Server listening on: {}:{}",
        server_config.service.host, server_config.service.port
    ));
    Logger::debug(&format!(
        "Allowed CORS origins: {:?}",
        server_config.service.cors
    ));
    Logger::debug(&format!("Database user: {}", server_config.database.user));

    Logger::info("Application initialized. Ready to handle requests!");
}
