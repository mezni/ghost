use config; // Import the config crate to reference config::ConfigError

/// A custom error type for the entire application.
/// This enum will consolidate various error types that can occur.
#[derive(Debug)]
pub enum AppError { // Renamed from MyAppError
    /// Represents an error specifically related to loading or parsing configuration.
    Configuration(config::ConfigError),
    // Add other application-specific error types here as your project grows.
    // For example:
    // Database(sqlx::Error),
    // Network(reqwest::Error),
    // Auth(String),
}

// Implement `std::fmt::Display` for `AppError` to allow easy printing.
impl std::fmt::Display for AppError { // Renamed from MyAppError
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::Configuration(err) => write!(f, "Configuration Error: {}", err),
            // Handle other error types here
        }
    }
}

// Implement `std::error::Error` trait for `AppError`.
// This allows `AppError` to be used with `?` operator and provides
// a common interface for error handling.
impl std::error::Error for AppError { // Renamed from MyAppError
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::Configuration(err) => Some(err),
            // Return source for other error types if they wrap another error
        }
    }
}

/// Implement `From` trait to convert `config::ConfigError` into `AppError`.
/// This allows using the `?` operator directly on results that return `config::ConfigError`,
/// automatically converting them into `AppError::Configuration`.
impl From<config::ConfigError> for AppError { // Renamed from MyAppError
    fn from(err: config::ConfigError) -> Self {
        AppError::Configuration(err)
    }
}
