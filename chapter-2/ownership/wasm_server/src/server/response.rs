use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct ResponseWasm {
    pub title: String,
    pub year: u16,
    pub id: String,
}

impl ResponseWasm {
    fn new(title: String, year: u16, id: String) -> Self {
        ResponseWasm { title, year, id }
    }
}
