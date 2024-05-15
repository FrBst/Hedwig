use core::str::FromStr;

use crate::error::AppError;

#[derive(Debug)]
pub enum Scheme {
    Http,
    Https,
}

impl Scheme {
    pub fn default_port(&self) -> u16 {
        match self {
            Scheme::Http => 80,
            Scheme::Https => 443,
        }
    }
}

impl std::fmt::Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::Http => write!(f, "http"),
            Scheme::Https => write!(f, "https"),
        }
    }
}

impl FromStr for Scheme {
    type Err = AppError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "http" => Ok(Scheme::Http),
            "https" => Ok(Scheme::Https),
            _ => Err(AppError::Scheme("This scheme is not supported".to_owned())),
        }
    }
}
