use tracing::{debug, error, info, span, warn, Level, Span};
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{
    fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry,
};
use opentelemetry::global;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use std::{io, path::PathBuf, time::Duration};

/// Logger configuration
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    pub level: String,
    pub format: LoggerFormat,
    pub file_path: Option<PathBuf>,
    pub enable_otel: bool,
    pub service_name: String,
}

/// Log format options
#[derive(Debug, Clone, Copy)]
pub enum LoggerFormat {
    Json,
    Pretty,
    Compact,
}

/// Logger initialization result
pub struct LoggerGuard {
    #[allow(dead_code)]
    file_guard: Option<WorkerGuard>,
}

impl LoggerConfig {
    /// Create default production config
    pub fn production() -> Self {
        Self {
            level: "info".to_string(),
            format: LoggerFormat::Json,
            file_path: None,
            enable_otel: true,
            service_name: "auth-service".to_string(),
        }
    }

    /// Create default development config
    pub fn development() -> Self {
        Self {
            level: "debug".to_string(),
            format: LoggerFormat::Pretty,
            file_path: None,
            enable_otel: false,
            service_name: "auth-service-dev".to_string(),
        }
    }
}

/// Initialize global logger
pub fn init(config: LoggerConfig) -> Result<LoggerGuard, Box<dyn std::error::Error>> {
    // Initialize OpenTelemetry if enabled
    if config.enable_otel {
        init_otel(&config.service_name)?;
    }

    // Create logging layer
    let (file_writer, file_guard) = if let Some(path) = config.file_path {
        let file_appender = tracing_appender::rolling::daily(path, "auth.log");
        let (writer, guard) = tracing_appender::non_blocking(file_appender);
        (Some(writer), Some(guard))
    } else {
        (None, None)
    };

    let fmt_layer = match config.format {
        LoggerFormat::Json => fmt::layer()
            .json()
            .with_writer(file_writer.unwrap_or_else(|| Box::new(io::stdout))),
        LoggerFormat::Pretty => fmt::layer()
            .pretty()
            .with_writer(file_writer.unwrap_or_else(|| Box::new(io::stdout))),
        LoggerFormat::Compact => fmt::layer()
            .compact()
            .with_writer(file_writer.unwrap_or_else(|| Box::new(io::stdout))),
    };

    // Configure log levels
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.level));

    // Initialize subscriber
    let subscriber = Registry::default()
        .with(env_filter)
        .with(fmt_layer);

    if config.enable_otel {
        let otel_layer = tracing_opentelemetry::layer();
        subscriber.with(otel_layer).init();
    } else {
        subscriber.init();
    }

    info!(service = config.service_name, "Logger initialized");

    Ok(LoggerGuard { file_guard })
}

/// Initialize OpenTelemetry tracer
fn init_otel(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    global::set_text_map_propagator(TraceContextPropagator::new());
    
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .with_trace_config(
            opentelemetry_sdk::trace::config()
                .with_resource(opentelemetry_sdk::Resource::new(vec![
                    opentelemetry::KeyValue::new("service.name", service_name.to_string()),
                ]))
        )
        .install_batch(opentelemetry_sdk::runtime::Tokio)?;

    Ok(())
}

/// Logging macros with context support
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        error!(error = ?$($arg)*, "Error occurred");
    };
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        warn!(error = ?$($arg)*, "Warning");
    };
}

/// Add context to current span
pub fn add_context<T: std::fmt::Debug>(key: &'static str, value: T) {
    Span::current().record(key, &tracing::field::debug(value));
}

/// Create a new span for operations
pub fn create_span(name: &'static str) -> Span {
    span!(Level::INFO, name)
}

/// Structured logging for HTTP requests
pub fn log_request(
    method: &str,
    path: &str,
    status: u16,
    latency: Duration,
    client_ip: &str,
) {
    info!(
        http.method = method,
        http.path = path,
        http.status_code = status,
        latency_ms = latency.as_millis(),
        client.ip = client_ip,
        "Request completed"
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::Instrument;

    #[test]
    fn test_logger_init() {
        let config = LoggerConfig {
            level: "debug".to_string(),
            format: LoggerFormat::Compact,
            file_path: None,
            enable_otel: false,
            service_name: "test-service".to_string(),
        };

        let _guard = init(config).unwrap();
        info!("Test log message");
    }

    #[tokio::test]
    async fn test_span_context() {
        let span = create_span("test_operation");
        async {
            add_context("user_id", "test_user");
            debug!("Inside span");
        }
        .instrument(span)
        .await;
    }
}