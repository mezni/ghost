use tracing::{debug, error, info, warn};
use tracing_subscriber::filter::EnvFilter;

pub struct Logger;

impl Logger {
    pub fn init() {
        // Use EnvFilter to support full RUST_LOG syntax (e.g., "info,my_crate=debug")
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new("info"));

        // Only initialize once; ignore error if already initialized (e.g., in tests)
        let _ = tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .with_target(false)
            .compact()
            .try_init();
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
