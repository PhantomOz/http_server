use crate::http::{HTTPRequest, HTTPResponse, ParseError, StatusCode};

use std::io::Read;
use std::net::TcpListener;

pub trait Handler {
    fn handle_request(&self, request: &HTTPRequest) -> HTTPResponse;
    fn handle_bad_request(&self, e: &ParseError) -> HTTPResponse {
        println!("Failed to parse request: {}", e);
        HTTPResponse::new(StatusCode::BadRequest, None)
    }
}

pub struct HTTPServer {
    address: String,
}

impl HTTPServer {
    pub fn new(address: String) -> Self {
        HTTPServer { address }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Listening on {}", self.address);

        let listener = TcpListener::bind(self.address).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("Connection established: {}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Request: {}", String::from_utf8_lossy(&buffer));

                            let response = match HTTPRequest::try_from(&buffer[..]) {
                                Ok(request) => handler.handle_request(&request),
                                Err(e) => handler.handle_bad_request(&e),
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }
                        }
                        Err(e) => {
                            println!("Error reading request: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Error accepting connection: {}", e);
                }
            }
        }
    }
}
