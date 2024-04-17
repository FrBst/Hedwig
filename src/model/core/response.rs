use http::{HeaderMap, HeaderValue};

use crate::model::http::http_status::HttpStatus;

#[derive(Debug)]
pub struct Response {
    body: String,
    status: HttpStatus,
    headers: HeaderMap<HeaderValue>,
}

impl Response {
    pub fn new(body: String, status: HttpStatus, headers: HeaderMap<HeaderValue>) -> Response {
        Response {
            body,
            status,
            headers,
        }
    }
}

