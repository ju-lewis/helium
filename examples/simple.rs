
use helium::{server::Server, http::Method};


fn main() -> std::io::Result<()> {

    // Create server
    let mut s = Server::new(8); // 8 threads allowed

    // Add routes
    s.route(Method::GET, "/".to_string(), || "Index route");

    // Run server
    s.run("127.0.0.1:8000")

}


