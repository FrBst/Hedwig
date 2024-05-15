use std::{error::Error, fmt};

#[derive(Debug)]
pub enum AppError {
    Method(String),
    Scheme(String),
    StatusCode(String),
    Request(String),
    Header(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Method(desc) => write!(f, "Method parsing error: {}", desc.as_str()),
            AppError::Scheme(desc) => write!(f, "Scheme parsing error: {}", desc.as_str()),
            AppError::StatusCode(desc) => {
                write!(f, "Status code name parsing error: {}", desc.as_str())
            }
            AppError::Request(desc) => write!(f, "Request error: {}", desc.as_str()),
            AppError::Header(desc) => write!(f, "Header error: {}", desc.as_str()),
        }
    }
}

impl Error for AppError {
    fn description(&self) -> &str {
        match self {
            AppError::Method(desc) => desc.as_str(),
            AppError::Scheme(desc) => desc.as_str(),
            AppError::StatusCode(desc) => desc.as_str(),
            AppError::Request(desc) => desc.as_str(),
            AppError::Header(desc) => desc.as_str(),
        }
    }
}
