
pub enum RagExceptions {
    ErrorWhileProcessing,
}

pub fn except_to_str(ex: RagExceptions) -> &'static str {
    match ex {
        RagExceptions::ErrorWhileProcessing => "[RAG | ERROR] - Error while processing",
    }
}
