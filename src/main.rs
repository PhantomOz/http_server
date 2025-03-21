use http_server::HTTPServer;
use std::env;
use website_handler::WebsiteHandler;

mod http;
mod http_server;
mod website_handler;

fn main() {
    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public path: {}", public_path);
    let http_server = HTTPServer::new("127.0.0.1:8080".to_string());
    http_server.run(WebsiteHandler::new(public_path));
}
