use crate::exceptions::RagExceptions;

pub fn rag_process(question: &str) -> Result<String, RagExceptions> {
    let vector = embed(question)?;
    let docs = query_qdrant(vector)?;
    let prompt = format_prompt(docs, question);
    let response = query_ollama(prompt)?;
    Ok(response)
}