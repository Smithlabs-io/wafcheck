use thiserror::Error;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("Environment variable missing: {0}")]
    MissingEnvVar(String),
    #[error("Cloudflare API request failed: {0}")]
    CloudflareApiError(String),
    #[error("API Request error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Header parsing error: {0}")]
    HeaderParse(#[from] reqwest::header::InvalidHeaderValue),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Other error: {0}")]
    Other(String),
}

impl AppError {
    pub fn exit_code(&self) -> i32 {
        match self {
            AppError::MissingEnvVar(_) => 1,
            AppError::CloudflareApiError(_) => 1,
            AppError::Reqwest(_) => 1,
            AppError::HeaderParse(_) => 1,
            AppError::Io(_) => 1,
            AppError::Json(_) => 1,
            AppError::Other(_) => 1,
        }
    }
}