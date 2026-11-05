use server::Server;
use http::Method;
use http::Request;

mod http;
mod server;
fn main() {
    let server: Server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}



