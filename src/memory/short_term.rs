// src/memory/short_term.rs
use std::collections::VecDeque;
use serde::{Serialize, Deserialize};
use crate::providers::deepseek::DeepSeekProvider;
use crate::completion::CompletionProvider; // Ensure this is imported

#[derive(Serialize, Deserialize)]
pub struct ShortTermMemory {
    messages: VecDeque<String>, // Stores recent messages
    max_size: usize,            // Maximum number of messages to store
}

impl ShortTermMemory {
    pub fn new(max_size: usize) -> Self {
        Self {
            messages: VecDeque::with_capacity(max_size),
            max_size,
        }
    }

    pub fn add_message(&mut self, message: String) {
        if self.messages.len() >= self.max_size {
            self.messages.pop_front(); // Remove the oldest message
        }
        self.messages.push_back(message);
    }

    pub fn get_context(&self) -> String {
        self.messages.iter().map(|s| s.as_str()).collect::<Vec<_>>().join("\n")
    }

    pub fn compress(&mut self) {
        if self.messages.len() > self.max_size {
            let excess = self.messages.len() - self.max_size;
            let compressed: Vec<String> = self.messages.drain(..excess).collect();
            let compressed_summary = format!("[Earlier conversation: {}]", compressed.join("; "));
            self.messages.push_front(compressed_summary);
        }
    }

    pub async fn summarize(&self, provider: &DeepSeekProvider) -> Result<String, Box<dyn std::error::Error>> {
        let conversation = self.get_context();
        let prompt = format!(
            "Summarize the following conversation in 3-5 sentences:\n{}",
            conversation
        );
        // Map the error from `CompletionError` to `Box<dyn std::error::Error>`
        provider.complete(&prompt).await.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
    }
}
