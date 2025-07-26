use crate::exceptions::RagExceptions;

mod core;
mod exceptions;
mod config;

fn main() {
    core::process::rag_process("How many people in Tokyo ?")
        .expect(RagExceptions::ErrorWhileProcessing.except_to_str());
}
