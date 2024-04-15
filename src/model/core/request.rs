use crate::model::{http::http_method::HttpMethod, request_headers::RequestHeaders};

use super::scheme::Scheme;

pub struct Request {
    pub method: HttpMethod,
    pub scheme: Scheme,
    pub domain: String,
    pub port: u16,
    pub path: Option<String>,
    pub query: Option<String>,
    pub headers: Option<RequestHeaders>,
}

impl Request {
    pub fn new(
        method: HttpMethod,
        scheme: Scheme,
        domain: String,
        port: u16,
        path: Option<String>,
        query: Option<String>,
        headers: Option<RequestHeaders>,
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
        if let Some(path) = &self.path {
            url.push_str(path);
        }
        if let Some(query) = &self.query {
            url.push_str(query);
        }
        url
    }

    pub fn build_host(&self) -> String {
        format!("{}:{}", self.domain, self.port)
    }
}
