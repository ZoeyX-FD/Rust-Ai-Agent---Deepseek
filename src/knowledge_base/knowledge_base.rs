// src/knowledge_base/knowledge_base.rs
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct KnowledgeEntry {
    pub keywords: Vec<String>,
    pub content: String,
}

pub struct KnowledgeBaseHandler {
    knowledge_base: Vec<KnowledgeEntry>,
}

impl KnowledgeBaseHandler {
    pub fn new(file_path: &str) -> Self {
        // Load the knowledge base from the file
        let knowledge_base = fs::read_to_string(file_path)
            .map_err(|e| {
                eprintln!("Failed to read knowledge base file: {}", e);
                std::process::exit(1);
            })
            .and_then(|data| {
                serde_json::from_str(&data).map_err(|e| {
                    eprintln!("Failed to parse knowledge base file: {}", e);
                    std::process::exit(1);
                })
            })
            .unwrap_or_else(|_| vec![]);

        Self {
            knowledge_base,
        }
    }

    pub fn retrieve_information(&self, query: &str) -> String {
        // Tokenize the query into words
        let query_words: Vec<_> = query.split_whitespace().map(|s| s.to_lowercase()).collect();

        // Find entries with matching keywords
        let relevant_entries: Vec<_> = self
            .knowledge_base
            .iter()
            .filter(|entry| {
                entry
                    .keywords
                    .iter()
                    .any(|kw| query_words.contains(&kw.to_lowercase()))
            })
            .collect();

        // Combine the content of relevant entries
        relevant_entries
            .iter()
            .map(|entry| entry.content.clone())
            .collect::<Vec<_>>()
            .join("\n")
    }
}