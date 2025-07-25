mod core;
mod exceptions;

fn main() {
    core::process::rag_process("How many people in Tokyo ?")
        .expect("[RAG | ERROR] - Error while processing");
}
