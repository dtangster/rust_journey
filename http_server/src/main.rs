mod http;

use http::method::Method;
use http::server::Server;

fn main() {
    let server = Server::new("127.0.0.1:8000".to_string());
    server.run();
}
