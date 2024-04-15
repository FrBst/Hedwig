use std::str::FromStr;

use hyper::HeaderMap;

pub struct RequestHeaders {
    map: HeaderMap,
}

impl RequestHeaders {
    pub fn new() -> RequestHeaders {
        Self {
            map: HeaderMap::new(),
        }
    }

    pub fn put(&mut self, key: &str, value: &str) {
        self.map.insert(
            hyper::header::HeaderName::from_str(key).unwrap(),
            value.parse().unwrap(),
        );
    }
}
