use tracing::{error, info, warn};
use tracing_subscriber;

pub struct Logger;

impl Logger {
    pub fn init() {
        tracing_subscriber::fmt()
            .with_target(false)
            .with_level(true)
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
}
