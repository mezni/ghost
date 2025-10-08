use std::str::FromStr;
use tracing::{Level, debug, error, info, warn};
use tracing_subscriber;

pub struct Logger;

impl Logger {
    pub fn init() {
        let env_log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());

        let level = Level::from_str(&env_log_level).unwrap_or(Level::INFO);

        tracing_subscriber::fmt()
            .with_target(false)
            .with_max_level(level)
            .compact()
            .init();
    }

    pub fn info(message: &str) {
        info!("{}", message);
    }

    pub fn error(message: &str) {
        error!("{}", message);
    }

    pub fn warn(message: &str) {
        warn!("{}", message);
    }

    pub fn debug(message: &str) {
        debug!("{}", message);
    }
}
