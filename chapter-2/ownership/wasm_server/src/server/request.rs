use serde::Deserialize;

#[derive(Deserialize)]
pub struct WasmRequest {
    pub wasm: Vec<u8>,
    pub run_function: String,
}
