use serde::Serialize;

#[derive(Serialize, Clone, Debug, Default)]
pub enum JobType {
    #[default]
    DELETE,
    MODIFY,
    SAVE
}
#[derive(Debug, Clone, Default, Serialize)]
struct JobMetaData {
    id: String,
    run_function: String,
    size: usize,
    job_type: JobType,
}

impl JobMetaData {
    fn new(id: String, run_function: String, size: usize, job_type: JobType) -> Self {
        Self {
            id,
            run_function,
            size,
            job_type,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct WasmJob {
    wasm: Vec<u8>,
    meta_data: JobMetaData,
}

impl WasmJob {
    pub fn new(wasm: Vec<u8>, id: String, run_function: String, job_type: JobType) -> Self {
        let size = wasm.len() * std::mem::size_of::<u8>();
        let meta_data = JobMetaData::new(id, run_function, size, job_type);
        Self { wasm, meta_data }
    }

    pub fn cloned_id(&self) -> String {
        self.meta_data.id.clone()
    }

    pub fn set_id(&mut self, id: String) {
        self.meta_data.id = id;
    }

    pub fn job_type(&self) -> JobType {
        self.meta_data.job_type.clone()
    }
}
