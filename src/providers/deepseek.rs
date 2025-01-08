use reqwest::Client;
use serde_json::json;
use crate::completion::{CompletionProvider, CompletionError};
use crate::personality::Personality;

pub struct DeepSeekProvider {
    api_key: String,
    client: Client,
    personality: Personality,
}

impl DeepSeekProvider {
    pub fn new(api_key: String, personality: Personality) -> Self {
        Self {
            api_key,
            client: Client::new(),
            personality, // Initialize with provided personality
        }
    }
}

#[async_trait::async_trait]
impl CompletionProvider for DeepSeekProvider {
    type Error = CompletionError;

    async fn complete(&self, prompt: &str) -> Result<String, Self::Error> {
        // Use the personality's system message
        let system_message = self.personality.system_message();

        let response = self.client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&json!({
                "model": "deepseek-chat",
                "messages": [
                    {"role": "system", "content": system_message}, // Use personality's system message
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 1000,
                "temperature": 0.7
            }))
            .send()
            .await?;

        // Check if the API request was successful
        if !response.status().is_success() {
            let error_message = response.text().await?;
            return Err(CompletionError::ApiError(error_message));
        }

        // Parse the response JSON
        let response_json = response.json::<serde_json::Value>().await?;

        // Extract the AI's response content
        let processed_data = response_json["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("No response from DeepSeek")
            .to_string();

        Ok(processed_data)
    }
}
