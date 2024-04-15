
use crate::model::http::http_status::HttpStatus;

#[derive(Debug)]
pub struct Response {
    body: String,
    status: HttpStatus
}

impl Response {
    pub fn new(body: String, status: HttpStatus) -> Response {
        Response {
            body,
            status
        }
    }
}