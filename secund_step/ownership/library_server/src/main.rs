use std::net::SocketAddr;

mod adapter;
mod book;
mod runner;
mod server;
use runner::Runner;
use server::{MyServer, RequestBook};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // create channel
    // numero de itens no canal                                                             👇🏼
    let (tx, rx) = mpsc::channel::<RequestBook>(1);

    // create Runner
    let mut runner = Runner::new(rx);

    // create server
    let server = MyServer::new(SocketAddr::from(([127, 0, 0, 1], 3000)));

    tokio::spawn(
        server.run(tx)
    );
    runner.run().await;

    Ok(())
}
