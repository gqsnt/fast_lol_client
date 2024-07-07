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