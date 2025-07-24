use thiserror::Error; 

#[derive(Error, Debug)]
pub enum IpWhoisError {
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("API returned an error: {0}")]
    ApiError(String),
}