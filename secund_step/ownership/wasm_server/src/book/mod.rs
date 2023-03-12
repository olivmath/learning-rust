use lazy_static::lazy_static;
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

lazy_static! {
    static ref DB: Arc<Mutex<HashMap<String, Wasm>>> = Arc::new(Mutex::new(HashMap::new()));
}

#[derive(Debug, Clone, Deserialize)]
pub struct Wasm {
    pub name: String,
    pub id: String,
    pub file: Vec<u8>,
    pub size: usize,
    pub main_function: String,
}

impl Wasm {
    pub fn new(name: String, id: String, file: Vec<u8>, main_function: String) -> Wasm {
        Wasm {
            name,
            id,
            file,
            size: file.len(),
            main_function,
        }
    }
}

impl std::fmt::Display for Wasm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) {}", self.id, self.main_function, self.size)
    }
}

pub struct Storage {
    wasms: Arc<Mutex<HashMap<String, Wasm>>>,
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

    pub fn add_wasm(&mut self, wasm: Wasm) -> Option<Wasm> {
        self.wasms.lock().unwrap().insert(wasm.id, wasm)
    }

    pub fn modify_wasm(&mut self, wasm: Wasm) -> Option<Wasm> {
        self.wasms.lock().unwrap().insert(wasm.id, wasm)
    }

    pub fn remove_wasm(&mut self, id: String) -> Option<Wasm> {
        self.wasms.lock().unwrap().remove(&id)
    }

    pub fn all_wasms(&self) -> Vec<Wasm> {
        self.wasms
            .lock()
            .unwrap()
            .iter()
            .map(|(_, v)| v.clone())
            .collect::<Vec<Wasm>>()
    }

    pub fn get_wasm(&self, id: String) -> Option<Wasm> {
        self.wasms.lock().unwrap().get(&id).cloned()
    }
}
