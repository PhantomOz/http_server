use http_server::HTTPServer;
use website_handler::WebsiteHandler;

mod http;
mod http_server;
mod website_handler;

fn main() {
    let http_server = HTTPServer::new("127.0.0.1:8080".to_string());
    http_server.run(WebsiteHandler);
}
