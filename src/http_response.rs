use crate::http_resource::HttpResource;
use chrono;
use http::StatusCode;
use std::{collections::HashMap, fmt};
pub struct HttpResponse {
    pub status_code: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new() -> HttpResponse {
        HttpResponse {
            status_code: String::new(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }
    pub fn from_resource(resource: &HttpResource) -> HttpResponse {
        let mut response = HttpResponse::new();
        response.add_header("Content-Type", &resource.mime_type);
        response.add_header(
            "Content-Length",
            resource.file_data.len().to_string().as_str(),
        );
        response.add_header("Server", "Bifrost");
        response.add_header("Date", chrono::Utc::now().to_rfc2822().as_str());
        response.body = resource.file_data.clone();
        response.set_status(http::StatusCode::OK);
        response
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
    pub fn to_bytes(&self) -> Vec<u8> {
        let head = self.headers_to_string();
        let body = self.body.iter();
        let data: Vec<u8> = head.as_bytes().iter().chain(body).copied().collect();
        data
    }

    fn headers_to_string(&self) -> String {
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
        tostring
    }
}

impl Default for HttpResponse {
    fn default() -> HttpResponse {
        HttpResponse::new()
    }
}
