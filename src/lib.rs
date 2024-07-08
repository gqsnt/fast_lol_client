pub mod ui;
pub mod config;
pub mod client;
pub mod utils;

#[derive(Debug, Clone)]
pub enum AppError {
    IoError(String),
    ReqwestError(String),
    RiotClientError(String),
    ApiRequestError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO Error: {}", e),
            AppError::ReqwestError(e) => write!(f, "Reqwest Error: {}", e),
            AppError::RiotClientError(e) => write!(f, "Riot Client Error: {}", e),
            AppError::ApiRequestError(e) => write!(f, "API Request Error: {}", e),
        }
    }
}


impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::IoError(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::ReqwestError(err.to_string())
    }
}


pub type AppResult<T> = Result<T, AppError>;