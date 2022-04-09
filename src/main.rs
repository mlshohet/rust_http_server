use server::Server;
use http::Method;
use http::Request;

// reference to an imported module
// module put in scope
mod server;
mod http;

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());

    // never returns
    // loops waiting for connections
    server.run();
}
