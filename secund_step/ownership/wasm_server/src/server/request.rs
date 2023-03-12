use serde::Serialize;

use crate::wasm::Wasm;

#[derive(Serialize, Clone, Debug)]
pub enum RequestType {
    SAVE,
    MODIFY,
    DELETE,
}

#[derive(Clone, Serialize, Debug)]
pub struct RequestWasm {
    pub req_type: RequestType,
    pub id: String,
    pub wasm: Wasm,
}

impl RequestWasm {
    pub fn new(id: String, wasm: Wasm, req_type: RequestType) -> Self {
        RequestWasm { req_type, id, wasm }
    }

    pub fn cloned_wasm(&self) -> Wasm {
        self.wasm.clone()
    }

    pub fn cloned_id(&self) -> String {
        self.id.clone()
    }

    pub fn from_request(id: String, req_type: RequestType) -> Self {
        RequestWasm {
            req_type,
            id,
            wasm: Wasm::default(),
        }
    }
}
