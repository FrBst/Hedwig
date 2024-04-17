#[derive(Debug)]
pub enum HttpStatus {
    Ok,
    Created,
    Accepted,
    MovedPermanently,
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
}

impl HttpStatus {
    pub fn get_code(&self) -> u16 {
        match *self {
            HttpStatus::Ok => 200,
            HttpStatus::Created => 201,
            HttpStatus::Accepted => 202,
            HttpStatus::MovedPermanently => 301,
            HttpStatus::BadRequest => 400,
            HttpStatus::Unauthorized => 401,
            HttpStatus::Forbidden => 403,
            HttpStatus::NotFound => 404,
        }
    }
}

impl From<hyper::StatusCode> for HttpStatus {
    fn from(value: hyper::StatusCode) -> Self {
        match value {
            hyper::StatusCode::OK => HttpStatus::Ok,
            hyper::StatusCode::CREATED => HttpStatus::Created,
            hyper::StatusCode::ACCEPTED => HttpStatus::Accepted,
            hyper::StatusCode::MOVED_PERMANENTLY => HttpStatus::MovedPermanently,
            hyper::StatusCode::BAD_REQUEST => HttpStatus::BadRequest,
            hyper::StatusCode::UNAUTHORIZED => HttpStatus::Unauthorized,
            hyper::StatusCode::FORBIDDEN => HttpStatus::Forbidden,
            hyper::StatusCode::NOT_FOUND => HttpStatus::NotFound,
            _ => panic!("Invalid status code {:#?}", value),
        }
    }
}

