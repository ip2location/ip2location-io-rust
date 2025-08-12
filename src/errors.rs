use thiserror::Error;

#[derive(Debug, Error)]
pub enum Ip2LocationError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),
    #[error("API error {status}: {message}")]
    ApiError { status: u16, message: String },
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("JSON parse error: {0}")]
    Json(#[from] serde_json::Error),
}
