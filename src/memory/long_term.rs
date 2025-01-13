// src/memory/long_term.rs
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use std::fs;
use chrono;

#[derive(Serialize, Deserialize)]
pub struct LongTermMemory {
    data: HashMap<String, String>, // Key-value pairs for persistent data
}

impl LongTermMemory {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn store(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }

    pub fn add_memory(&mut self, user_input: &str, ai_response: &str) {
        let key = format!("memory_{}", chrono::Utc::now().timestamp());
        let value = format!("User: {}\nAssistant: {}", user_input, ai_response);
        self.store(key, value);
    }

    pub fn retrieve(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }

    pub fn save_to_file(&self, path: &str) -> std::io::Result<()> {
        let serialized = serde_json::to_string(&self.data)?;
        fs::write(path, serialized)
    }

    pub fn load_from_file(path: &str) -> std::io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let data: HashMap<String, String> = serde_json::from_str(&data)?;
        Ok(Self { data })
    }
}
