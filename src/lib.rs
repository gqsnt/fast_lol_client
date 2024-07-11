pub mod ui;
pub mod config;
pub mod client;
pub mod utils;

#[derive(Debug, Clone)]
pub enum AppError {
    IoError(String),
    ReqwestError(String),
    ParsingError(String),
    ApiRequestError(String),
    DisconnectedError(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::IoError(e) => write!(f, "IO Error: {}", e),
            AppError::DisconnectedError(e) => write!(f, "Client Disconnected: {}", e),
            AppError::ParsingError(e) => write!(f, "Parsing Error: {}", e),
            AppError::ReqwestError(e) =>  write!(f, "Reqwest Error: {}", e),
            AppError::ApiRequestError(e) =>  write!(f, "Api Request Error: {}", e),
        }
    }
}

macro_rules! impl_from_error {
    ($error:ty, $variant:ident) => {
        impl From<$error> for AppError {
            fn from(err: $error) -> Self {
                AppError::$variant(err.to_string())
            }
        }
    };
}

impl_from_error!(std::io::Error, IoError);
impl_from_error!(reqwest::Error, ReqwestError);



pub type AppResult<T> = Result<T, AppError>;