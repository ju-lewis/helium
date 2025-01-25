
use helium::{server::Server, Method};


fn main() -> std::io::Result<()> {

    // Create server
    let mut s = Server::new(8); // 8 threads allowed

    // Add routes
    s.route(Method::GET, "/", || "Index route".to_string());

    // Run server
    s.run()

}


