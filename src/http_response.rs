use std::fmt::Display;
use std::{collections::HashMap, fmt};

use http::StatusCode;
pub struct HttpResponse {
    pub status_code: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse {
            status_code: String::new(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(String::from(key), String::from(value));
    }
    pub fn set_status(&mut self, code: StatusCode) {
        let mut temp = String::new();
        temp += "HTTP/1.1 ";
        temp += code.as_str();
        temp += " ";
        temp += code.canonical_reason().unwrap_or("");
        self.status_code = temp;
    }
}

impl Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tostring = String::new();
        tostring += self.status_code.as_str();
        tostring += "\r\n";
        for (header, value) in &self.headers {
            tostring += header.as_str();
            tostring += ": ";
            tostring += value.as_str();
            tostring += "\r\n";
        }
        tostring += "\r\n";
        tostring += self.body.as_str();
        write!(f, "{}", tostring)
    }
}
