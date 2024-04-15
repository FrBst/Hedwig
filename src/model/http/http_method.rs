use std::{fmt::Display, str::FromStr, string::ParseError};

#[derive(Debug)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Patch,
    Delete
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FromStr for HttpMethod {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HttpMethod::Get)
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
        }
    }
}

impl Into<hyper::Method> for HttpMethod {
    fn into(self) -> hyper::Method {
        match self {
            Self::Get => hyper::Method::GET,
            Self::Post => hyper::Method::POST,
            Self::Put => hyper::Method::PUT,
            Self::Patch => hyper::Method::PATCH,
            Self::Delete => hyper::Method::DELETE,
        }
    }
}