mod domain;
mod routes;

use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use routes::routing;
use std::convert::Infallible;
use std::net::SocketAddr;

fn clear_terminal_with(string: &str) {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    println!("{}", string);
}

#[tokio::main]
async fn main() {
    println!("🔗 Binding server in http://127.0.0.1:3000");
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // A `Service` is needed for every connection, so this
    // creates one from our `handler` function.
    let make_service = make_service_fn(|_connection| async {
        // service_fn converts our function into a `Service`
        Ok::<_, Infallible>(service_fn(routing))
    });

    let server = Server::bind(&addr).serve(make_service);

    clear_terminal_with("🚀 Run Rustriz");
    match server.await {
        Ok(_) => println!("server started"),
        Err(e) => eprintln!("{}", e),
    }
}
