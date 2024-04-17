use std::str::FromStr;

use url::Url;

use crate::{
    error::AppError,
    model::{
        core::{request::Request, response::Response, scheme::Scheme},
        http::http_method::HttpMethod,
        request_headers::RequestHeaders,
    },
    requester,
};

pub struct Client {
    url: String,
    follow: bool,
    strict_url: bool,
}

impl Client {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            follow: false,
            strict_url: false,
        }
    }

    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = follow;
        self
    }

    pub fn strict_url(mut self, strict_url: bool) -> Self {
        self.strict_url = strict_url;
        self
    }

    pub fn send(&mut self) -> Result<Response, AppError> {
        let request = self.build_request();

        dbg!(&request);
        requester::get::send_request(request)
    }

    fn build_request(&mut self) -> Request {
        if !self.strict_url {
            self.fix_url();
        }
        let url = Url::parse(self.url.as_str()).unwrap();

        let scheme = Scheme::from_str(url.scheme()).unwrap();
        let domain = url.domain().unwrap().to_owned();
        let method = HttpMethod::Get;
        let port = url.port().unwrap_or(scheme.default_port());
        let path = url.path().to_owned();
        let query = url.query().map(|s| s.to_owned());
        let headers = RequestHeaders::new();
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

    fn fix_url(&mut self) {
        if !self.url.contains("://") {
            self.url.insert_str(0, "http://");
        }
    }
}
