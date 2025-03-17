use crate::http::StatusCode;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct HTTPResponse {
    status_code: StatusCode,
    body: Option<String>,
}

impl HTTPResponse {
    pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
        HTTPResponse {
            status_code,
            body: body,
        }
    }
}

impl Display for HTTPResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            f,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
    }
}
