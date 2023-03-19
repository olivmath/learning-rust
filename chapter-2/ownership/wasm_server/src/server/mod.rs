mod handler;
pub mod request;
mod response;
mod utils;

use super::domain::job::WasmJob;
use super::server::{handler::handler_routes, utils::clear_terminal_with};
use hyper::service::{make_service_fn, service_fn};
use hyper::Server;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

pub struct MyServer {
    addr: SocketAddr,
}

impl MyServer {
    pub fn new(addr: SocketAddr) -> MyServer {
        MyServer { addr }
    }

    pub async fn run(self, tx: Sender<WasmJob>) -> Result<(), Infallible> {
        clear_terminal_with("🔗 Binding server in http://127.0.0.1:3000");

        let make_service = make_service_fn(move |_connection| {
            let _tx = tx.clone();
            async { Ok::<_, Infallible>(service_fn(move |req| handler_routes(_tx.clone(), req))) }
        });

        let server = Server::bind(&self.addr).serve(make_service);

        println!("🚀 Run Library Server 📚");

        if let Err(e) = server.await {
            eprintln!("{}", e);
        }
        Ok(())
    }
}
