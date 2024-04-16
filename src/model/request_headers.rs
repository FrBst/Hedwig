use std::collections::HashMap;

#[derive(Debug)]
pub struct RequestHeaders {
    map: HashMap<String, String>,
}

impl RequestHeaders {
    pub fn new() -> RequestHeaders {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn put(&mut self, key: &str, value: &str) {
        self.map.insert(key.to_owned(), value.to_owned());
    }
}

impl From<Vec<(String, String)>> for RequestHeaders {
    fn from(headers: Vec<(String, String)>) -> Self {
        let mut map = HashMap::new();
        for header in headers {
            map.insert(header.0, header.1);
        }
        Self { map }
    }
}
