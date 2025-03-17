use super::http_server::Handler;
use crate::http::{HTTPRequest, HTTPResponse, StatusCode};

pub struct WebsiteHandler;

impl Handler for WebsiteHandler {
    fn handle_request(&self, request: &HTTPRequest) -> HTTPResponse {
        HTTPResponse::new(StatusCode::Ok, Some("<h1>Hello, world!</h1>".to_string()))
    }
}
