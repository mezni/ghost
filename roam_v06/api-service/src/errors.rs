use config; // Still needed to reference config::ConfigError
use thiserror::Error; // Import the Error derive macro

/// A custom error type for the entire application.
/// `thiserror::Error` automatically implements Display, Debug, and std::error::Error.
#[derive(Error, Debug)] // Derive Error (which includes Debug)
pub enum AppError {
    /// Represents an error specifically related to loading or parsing configuration.
    ///
    /// The `#[error(...)]` attribute defines the display message for this variant.
    /// The `#[from]` attribute automatically implements `From<config::ConfigError> for AppError`.
    #[error("Configuration Error: {0}")]
    Configuration(#[from] config::ConfigError), // #[from] automatically handles conversion

    // Add other application-specific error types here as your project grows.
    // #[error("Database error: {0}")]
    // Database(#[from] sqlx::Error),
    //
    // #[error("Network request failed: {0}")]
    // Network(#[from] reqwest::Error),
    //
    // #[error("Authentication failed: {0}")]
    // Auth(String),
}