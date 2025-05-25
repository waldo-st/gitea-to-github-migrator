use reqwest::StatusCode;
use thiserror::Error;
use tokio::sync::AcquireError;

#[derive(Debug, Error)]
pub enum GiteaError {
    #[error("JSON parsing failed: {0}")]
    ParseError(#[from] serde_json::Error),
    #[error("Concurrency error: {0}")]
    SemaphoreError(#[from] AcquireError),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("API request failed {0}")]
    ApiError(String),
    #[error("Authentication failed {0}")]
    AuthError(String),
    #[error("Repository not found {0}")]
    NotFound(String),
    #[error("File not found {0}")]
    IoError(#[from] std::io::Error),
    #[error("Error git {0}")]
    GitError(String),
    #[error("Error Missing Argument")]
    MissingArguments,
    #[error("Error Dialoguer {0}")]
    DialoguerError(String),
}

impl From<reqwest::Error> for GiteaError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_status() {
            match error.status() {
                Some(StatusCode::UNAUTHORIZED) => GiteaError::AuthError(error.to_string()),
                Some(StatusCode::NOT_FOUND) => GiteaError::NotFound(error.to_string()),
                _ => GiteaError::ApiError(error.to_string()),
            }
        } else {
            GiteaError::NetworkError(error.to_string())
        }
    }
}
