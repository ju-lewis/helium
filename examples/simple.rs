
extern crate helium;

fn main() -> std::io::Result<()> {

    // Create server
    let s = Server::new(8); // 8 threads allowed

    // Add routes
    s.route(helium::GET, "/", async || "Index route".to_string());

    // Run server
    s.run()

}


