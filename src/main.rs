#![allow(unused)]
mod server;
mod http;
mod handler;

use handler::HttpHandler;

fn main() {
    println!("Hello!");
    let addr = String::from("127.0.0.1:8080");
    let server = server::Server::new(addr);
    let website_path = String::from("public");
    println!("Website path: {}", website_path);
    server.run(HttpHandler::new(website_path));
}
