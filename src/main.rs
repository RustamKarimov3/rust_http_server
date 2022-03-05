#![allow(dead_code)]

use server::Server;

mod http;
mod server;

fn main() {
    let server: Server = Server::new("127.0.0.1:8080".to_string());

    server.start();
}





