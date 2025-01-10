use thiserror::Error;
use reqwest::StatusCode;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("HTTP error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("Server returned status code {0}")]
    StatusCodeError(StatusCode),

    #[error("Invalid or unexpected data: {0}")]
    DataError(String),

    #[error("Other error: {0}")]
    Other(String),
}
