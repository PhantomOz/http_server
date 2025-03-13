use http_server::HTTPServer;

fn main() {
    let http_server = HTTPServer::new("0.0.0.0:8080".to_string());
    http_server.run();
}

mod http_server {
    pub struct HTTPServer {
        address: String,
    }

    impl HTTPServer {
        pub fn new(address: String) -> Self {
            HTTPServer { address }
        }

        pub fn run(self) {
            println!("Listening on {}", self.address);
        }
    }
}

mod http {
    pub mod request {
        use super::http_method::HTTPMethod;
        pub struct Request {
            method: HTTPMethod,
            path: String,
            query_params: Option<String>,
        }
    }
    
    pub mod http_method {
        pub enum HTTPMethod {
            GET,
            POST,
            PUT,
            DELETE,
            PATCH,
            OPTIONS,
            HEAD,
            TRACE,
            CONNECT,
        }
    }
}
