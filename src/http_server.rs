use crate::http::http_request::HTTPRequest;
use std::io::{Read, Write};
use std::net::TcpListener;

pub struct HTTPServer {
    address: String,
}

impl HTTPServer {
    pub fn new(address: String) -> Self {
        HTTPServer { address }
    }

    pub fn run(self) {
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

                            match HTTPRequest::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                }
                                Err(e) => {
                                    println!("Error parsing request: {}", e);
                                }
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
