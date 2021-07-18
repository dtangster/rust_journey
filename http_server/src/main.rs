mod http;
mod server;

use http::method::Method;
use server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}
