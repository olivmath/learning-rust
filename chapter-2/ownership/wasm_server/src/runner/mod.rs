use crate::domain::{job::WasmJob, wasm::Wasm};

use super::{
    domain::{job::JobType, Storage},
    server::request::WasmRequest,
};

use tokio::sync::mpsc::Receiver;

pub struct Runner {
    rx: Receiver<WasmJob>,
    storage: Storage,
}

impl Runner {
    pub fn new(rx: Receiver<WasmJob>) -> Self {
        Runner {
            rx,
            storage: Storage::new(),
        }
    }

    pub async fn run(&mut self) {
        while let Some(wasm_job) = self.rx.recv().await {
            match wasm_job.job_type() {
                JobType::SAVE => {
                    self.storage.add_wasm(wasm_job);
                }
                JobType::MODIFY => {
                    self.storage.modify_wasm(wasm_job);
                }
                JobType::DELETE => {
                    self.storage.remove_wasm(wasm_job.cloned_id());
                }
            }
        }
    }
}
