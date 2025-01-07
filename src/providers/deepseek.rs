// src/providers/deepseek.rs
use reqwest::Client;
use serde_json::json;
use crate::completion::{CompletionProvider, CompletionError};

pub struct DeepSeekProvider {
    api_key: String,
    client: Client,
}

impl DeepSeekProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }
}

#[async_trait::async_trait]
impl CompletionProvider for DeepSeekProvider {
    type Error = CompletionError;

    async fn complete(&self, prompt: &str) -> Result<String, Self::Error> {
        let response = self.client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "deepseek-chat",
                "messages": [
                    {"role": "system", "content": "You are a helpful assistant."},
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 1000,
                "temperature": 0.7
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_message = response.text().await?;
            return Err(CompletionError::ApiError(error_message));
        }

        let response_json = response.json::<serde_json::Value>().await?;
        let processed_data = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response from DeepSeek")
            .to_string();

        Ok(processed_data)
    }
}
