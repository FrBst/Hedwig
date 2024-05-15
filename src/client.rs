use std::str::FromStr;

use url::Url;

use crate::{
    error::AppError,
    model::{
        core::{request::Request, response::Response, scheme::Scheme},
        http::{http_method::HttpMethod, http_status::HttpStatus},
        request_headers::RequestHeaders,
    },
    sender::send_request,
};

pub struct Client {
    url: String,
    follow: bool,
    strict_url: bool,
    headers: Option<RequestHeaders>,
}

impl Client {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            follow: false,
            strict_url: false,
            headers: None,
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

    pub fn headers(mut self, headers: Vec<String>) -> Result<Self, AppError> {
        self.headers = Some(Client::parse_headers(headers)?);
        Ok(self)
    }

    pub fn send(&mut self) -> Result<Response, AppError> {
        let mut request = self.build_request()?;

        dbg!(&request);
        let resp = send_request(&request);
        dbg!(&resp);
        let mut resp = resp?;
        while self.follow && resp.status == HttpStatus::MovedPermanently {
            self.url = resp
                .headers
                .get("Location")
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            request = self.build_request()?;
            dbg!(&request);
            resp = send_request(&request)?;
            dbg!(&resp);
        }

        Ok(resp)
    }

    fn build_request(&mut self) -> Result<Request, AppError> {
        if !self.strict_url {
            self.fix_url();
        }
        let url = Url::parse(self.url.as_str()).map_err(|e| AppError::Request(e.to_string()))?;

        let scheme =
            Scheme::from_str(url.scheme()).map_err(|e| AppError::Request(e.to_string()))?;
        let domain = if let Some(host) = url.host_str() {
            host.to_owned()
        } else {
            return Err(AppError::Request("No host in URL".to_owned()));
        };
        let method = HttpMethod::Get;
        let port = url.port();
        let path = url.path().to_owned();
        let query = url.query().map(|s| s.to_owned());
        let mut headers = RequestHeaders::new();
        headers.put("User-Agent", "insucknia/0.0.1");
        headers.put("Accept", "*/*");
        Ok(Request {
            method,
            scheme,
            domain,
            port,
            path,
            query,
            headers,
        })
    }

    fn fix_url(&mut self) {
        if !self.url.contains("://") {
            self.url.insert_str(0, "http://");
        }
    }

    fn parse_headers(headers: Vec<String>) -> Result<RequestHeaders, AppError> {
        let mut res = Vec::new();
        for h in headers {
            let mut parts = h.split(':');
            let key = parts.next();
            let value = parts.next();

            if let (Some(key), Some(value)) = (key, value) {
                res.push((key.to_owned(), value.to_owned()));
            } else {
                return Err(AppError::Header("Could not parse headers".to_owned()));
            }
        }
        Ok(res.into())
    }
}
