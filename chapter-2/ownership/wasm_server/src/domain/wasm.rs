use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Wasm {
    pub name: String,
    pub id: String,
    pub file: Vec<u8>,
    pub size: usize,
    pub main_function: String,
}

impl Wasm {
    pub fn new(name: String, id: String, file: Vec<u8>, main_function: String) -> Wasm {
        let size = file.len();
        Wasm {
            name,
            id,
            file,
            size,
            main_function,
        }
    }

    pub fn default() -> Wasm {
        Wasm {
            name: "".into(),
            id: "".into(),
            file: "".into(),
            size: 0,
            main_function: "".into(),
        }
    }

    pub fn cloned_id(&self) -> String {
        self.id.clone()
    }
}

impl std::fmt::Display for Wasm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}) {}", self.id, self.main_function, self.size)
    }
}
