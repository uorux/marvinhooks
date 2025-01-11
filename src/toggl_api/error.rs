use reqwest::StatusCode;
use thiserror::Error;

/// Errors that can occur when interacting with the Toggl Track API
#[derive(Debug, Error)]
pub enum TogglError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Server returned status code {0}")]
    StatusCodeError(StatusCode),

    #[error("Invalid or unexpected data: {0}")]
    DataError(String),

    #[error("Other error: {0}")]
    Other(String),
}
