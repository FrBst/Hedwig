use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AppError {
    Method,
    Scheme,
    StatusCode,
    Request,
    Header,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AppError::Method => write!(f, "Method parsing error"),
            AppError::Scheme => write!(f, "Scheme parsing error"),
            AppError::StatusCode => write!(f, "Status code name parsing error"),
            AppError::Request => write!(f, "Request error"),
            AppError::Header => write!(f, "Header error"),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match *self {
            AppError::Method => "MethodError",
            AppError::Scheme => "SchemeError",
            AppError::StatusCode => "StatusCodeError",
            AppError::Request => "RequestError",
            AppError::Header => "HeaderError",
        }
    }
}
