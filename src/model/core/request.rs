use std::borrow::Borrow;

use crate::model::{http::http_method::HttpMethod, request_headers::RequestHeaders};

use super::scheme::Scheme;

#[derive(Debug)]
pub struct Request {
    pub method: HttpMethod,
    pub scheme: Scheme,
    pub domain: String,
    pub port: u16,
    pub path: String,
    pub query: Option<String>,
    pub headers: RequestHeaders,
}

impl Request {
    pub fn new(
        method: HttpMethod,
        scheme: Scheme,
        domain: String,
        port: u16,
        path: String,
        query: Option<String>,
        headers: RequestHeaders,
    ) -> Request {
        Request {
            method,
            scheme,
            domain,
            port,
            path,
            query,
            headers,
        }
    }

    pub fn build_url(&self) -> String {
        let mut url = format!("{}://{}:{}", self.scheme, self.domain, self.port);
        url.push_str(self.path.borrow());
        if let Some(query) = &self.query {
            url.push_str(query);
        }
        url
    }

    pub fn build_host(&self) -> String {
        format!("{}:{}", self.domain, self.port)
    }
}
