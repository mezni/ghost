use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Initialize basic logging for the service
pub fn init_telemetry(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::new(std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()));

    // Basic logging setup
    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().with_target(false));

    subscriber.init();

    tracing::info!("Logging initialized for service: {}", service_name);
    Ok(())
}

/// Create a simple span for manual instrumentation
pub fn create_span(span_name: &str) -> tracing::Span {
    tracing::info_span!("app", name = span_name)
}

/// Log an error with context
pub fn log_error(error: &impl std::error::Error, context: &str) {
    tracing::error!(error = %error, context = context, "Operation failed");
}

/// Log a warning with context  
pub fn log_warn(message: &str, context: &str) {
    tracing::warn!(message = message, context = context, "Warning");
}

/// Log info with context
pub fn log_info(message: &str, context: &str) {
    tracing::info!(message = message, context = context, "Info");
}

/// No-op shutdown for compatibility
pub fn shutdown_telemetry() {
    // No-op in basic implementation
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_telemetry() {
        let result = init_telemetry("test-service");
        assert!(result.is_ok());
    }

    #[test]
    fn test_span_creation() {
        let span = create_span("test-operation");
        assert_eq!(span.name(), "app");
    }

    #[test]
    fn test_log_helpers() {
        // These should not panic
        log_info("test message", "test context");
        log_warn("test warning", "test context");

        let test_error = std::io::Error::new(std::io::ErrorKind::Other, "test error");
        log_error(&test_error, "test context");
    }
}
