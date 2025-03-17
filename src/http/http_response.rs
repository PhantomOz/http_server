use crate::http::StatusCode;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::io::{Result as IoResult, Write};
use std::net::TcpStream;

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

    pub fn send(&self, stream: &mut TcpStream) -> IoResult<()> {
        let body = match &self.body {
            Some(b) => b,
            None => "",
        };

        write!(
            stream,
            "HTTP/1.1 {} {}\r\n\r\n{}",
            self.status_code,
            self.status_code.reason_phrase(),
            body
        )
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
