// src/main.rs

mod errors;
mod infra; // Declares the 'infra' module // Declares the top-level 'errors' module
use errors::AppError;
use infra::logger::Logger; // Import AppError directly from the top-level errors module

fn main() {
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
