use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),
    
    #[error("Path is not a directory: {0}")]
    NotADirectory(String),
    
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Parse error: {0}")]
    Parse(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Authentication error: {0}")]
    Auth(String),
    
    #[error("Permission denied: {0}")]
    Permission(String),
    
    #[error("Unknown error: {0}")]
    Unknown(String),
}