use std::borrow::Borrow;

use crate::model::{http::http_method::HttpMethod, request_headers::RequestHeaders};

use super::scheme::Scheme;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub scheme: Scheme,
    pub domain: String,
    pub port: Option<u16>,
    pub path: String,
    pub query: Option<String>,
    pub headers: RequestHeaders,
}

impl Request {
    pub fn build_url(&self) -> String {
        let mut url = format!("{}://{}{}", self.scheme, self.domain, self.path);
        if let Some(query) = &self.query {
            url.push_str(query);
        }
        url
    }

    pub fn build_host(&self) -> String {
        let mut url = self.domain.clone();
        if let Some(port) = self.port {
            url.push_str(&format!(":{}", port));
        }
        url
    }

    pub fn build_host_with_port(&self) -> String {
        let url = format!(
            "{}:{}",
            self.domain.clone(),
            self.port.unwrap_or(self.scheme.default_port())
        );
        url
    }
}
