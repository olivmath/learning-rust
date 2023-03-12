use crate::wasm::Storage;

use super::server::request::{RequestType, RequestWasm};
use tokio::sync::mpsc::Receiver;

pub struct Runner {
    rx: Receiver<RequestWasm>,
    storage: Storage,
}

impl Runner {
    pub fn new(rx: Receiver<RequestWasm>) -> Self {
        Runner {
            rx,
            storage: Storage::new(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(req_wasm) = self.rx.recv().await {
            match req_wasm.req_type {
                RequestType::SAVE => {
                    self.storage.add_wasm(req_wasm.cloned_wasm());
                }
                RequestType::MODIFY => {
                    self.storage.modify_wasm(req_wasm.cloned_wasm());
                }
                RequestType::DELETE => {
                    self.storage.remove_wasm(req_wasm.cloned_id());
                }
            }
        }
    }
}
