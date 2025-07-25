
#[derive(Debug)]
pub enum RagExceptions {
    ErrorWhileProcessing,
}

impl RagExceptions {
    pub fn except_to_str(&self) -> &'static str {
        match self {
            RagExceptions::ErrorWhileProcessing => "[RAG | ERROR] - Error while processing",
        }
    }
}
