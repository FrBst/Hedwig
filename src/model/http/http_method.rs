use std::{fmt::Display, str::FromStr};

use crate::error::AppError;

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Options,
    Head,
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for HttpMethod {
    type Err = AppError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(HttpMethod::Get),
            "POST" => Ok(HttpMethod::Post),
            "PUT" => Ok(HttpMethod::Put),
            "PATCH" => Ok(HttpMethod::Patch),
            "DELETE" => Ok(HttpMethod::Delete),
            "OPTIONS" => Ok(HttpMethod::Options),
            "HEAD" => Ok(HttpMethod::Head),
            _ => Err(AppError::Method(
                "This HTTP method is not supported".to_string(),
            )),
        }
    }
}

impl Clone for HttpMethod {
    fn clone(&self) -> Self {
        match self {
            Self::Get => Self::Get,
            Self::Post => Self::Post,
            Self::Put => Self::Put,
            Self::Patch => Self::Patch,
            Self::Delete => Self::Delete,
            Self::Options => Self::Options,
            Self::Head => Self::Head,
        }
    }
}

impl From<HttpMethod> for hyper::Method {
    fn from(value: HttpMethod) -> Self {
        match value {
            HttpMethod::Get => hyper::Method::GET,
            HttpMethod::Post => hyper::Method::POST,
            HttpMethod::Put => hyper::Method::PUT,
            HttpMethod::Patch => hyper::Method::PATCH,
            HttpMethod::Delete => hyper::Method::DELETE,
            HttpMethod::Options => hyper::Method::OPTIONS,
            HttpMethod::Head => hyper::Method::HEAD,
        }
    }
}
