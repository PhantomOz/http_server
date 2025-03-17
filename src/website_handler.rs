use super::http_server::Handler;
use crate::http::{HTTPMethod, HTTPRequest, HTTPResponse, StatusCode};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);
        match std::fs::read_to_string(path) {
            Ok(content) => Some(content),
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &HTTPRequest) -> HTTPResponse {
        match request.method() {
            HTTPMethod::GET => match request.path() {
                "/" => HTTPResponse::new(StatusCode::Ok, self.read_file("index.html")),
                "/hello" => HTTPResponse::new(StatusCode::Ok, self.read_file("hello.html")),
                _ => HTTPResponse::new(StatusCode::NotFound, None),
            },
            _ => HTTPResponse::new(StatusCode::MethodNotAllowed, None),
        }
    }
}
