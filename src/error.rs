use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Auth(String),

    #[error("API error: {0}")]
    Api(String),

    #[error("API error ({status}): {message}")]
    ApiStatus { status: u16, message: String },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Rate limited: retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::Auth(_) => 2,
            AppError::Api(_) => 3,
            AppError::ApiStatus { .. } => 3,
            AppError::Config(_) => 4,
            AppError::NotFound(_) => 5,
            AppError::RateLimited { .. } => 6,
            AppError::Http(_) => 7,
            AppError::Io(_) => 8,
            AppError::Serialization(_) => 9,
        }
    }
}
