use core::errors::AppError;
use core::logger::Logger;

use std::process;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    Logger::init();

    let srv_config = match read_srv_config() {
        Ok(cfg) => {
            Logger::info("Config Server - Loaded");
            cfg
        }
        Err(e) => {
            Logger::error(&format!("Config Server - Failed : {:?}", e));
            Logger::info("Stop.");
            process::exit(1);
        }
    };

    Logger::info("Start.");
    Logger::info("Stop.");
    Ok(())
}
