fn main() {
    let http_server = HTTPServer::new("0.0.0.0:8080".to_string());
    http_server.run();
}

struct HTTPServer {
    address: String,
}

impl HTTPServer {
    fn new(address: String) -> Self {
        HTTPServer { address }
    }

    fn run(self) {
        println!("Listening on {}", self.address);
    }
}