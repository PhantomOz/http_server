use http_server::HTTPServer;
use http::http_request::HTTPRequest;

mod http_server;
mod http;

fn main() {
    let http_server = HTTPServer::new("127.0.0.1:8080".to_string());
    http_server.run();
}
