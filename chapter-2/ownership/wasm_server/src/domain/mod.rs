pub mod job;
pub mod wasm;

use lazy_static::lazy_static;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use self::job::WasmJob;

lazy_static! {
    static ref DB: Arc<Mutex<HashMap<String, WasmJob>>> = Arc::new(Mutex::new(HashMap::new()));
}
pub struct Storage {
    wasms: Arc<Mutex<HashMap<String, WasmJob>>>,
}

impl Storage {
    pub fn new() -> Storage {
        Storage {
            wasms: Arc::clone(&DB),
        }
    }

    pub fn len(&self) -> usize {
        self.wasms.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.wasms.lock().unwrap().is_empty()
    }

    pub fn add_wasm(&mut self, wasm: WasmJob) -> Option<WasmJob> {
        self.wasms.lock().unwrap().insert(wasm.cloned_id(), wasm)
    }

    pub fn modify_wasm(&mut self, wasm: WasmJob) -> Option<WasmJob> {
        self.wasms.lock().unwrap().insert(wasm.cloned_id(), wasm)
    }

    pub fn remove_wasm(&mut self, id: String) -> Option<WasmJob> {
        self.wasms.lock().unwrap().remove(&id)
    }

    pub fn all_wasms(&self) -> Vec<WasmJob> {
        self.wasms
            .lock()
            .unwrap()
            .iter()
            .map(|(_, v)| v.clone())
            .collect::<Vec<WasmJob>>()
    }

    pub fn get_wasm(&self, id: String) -> Option<WasmJob> {
        self.wasms.lock().unwrap().get(&id).cloned()
    }
}
