// logger.rs
use tracing::{error, info, warn};
use tracing_subscriber;

pub struct Logger;

impl Logger {
    /// Initialize the logger with a compact formatter.
    pub fn init() {
        tracing_subscriber::fmt()
            .with_target(false)
            .with_level(true)
            .compact()
            .init();
    }

    /// Log an info level message.
    pub fn info(message: &str) {
        info!("{}", message);
    }

    /// Log an error level message.
    pub fn error(message: &str) {
        error!("{}", message);
    }

    /// Log a warning level message.
    pub fn warn(message: &str) {
        warn!("{}", message);
    }
}
