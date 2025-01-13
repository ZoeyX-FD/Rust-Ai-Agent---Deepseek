use reqwest::Client;
use serde_json::json;
use std::error::Error;
use colored::Colorize;
use crate::completion::CompletionProvider;

pub struct DeepSeekProvider {
    api_key: String,
    client: Client,
    system_message: String,
}

impl CompletionProvider for DeepSeekProvider {
    type Error = Box<dyn Error + Send + Sync>;

    async fn complete(&self, prompt: &str) -> Result<String, Self::Error> {
        let response = self.client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&json!({
                "model": "deepseek-chat",
                "messages": [
                    {
                        "role": "system",
                        "content": self.system_message
                    },
                    {
                        "role": "user",
                        "content": prompt
                    }
                ]
            }))
            .send()
            .await?;

        let result = response.json::<serde_json::Value>().await?;
        
        let content = result["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("")
            .to_string();

        Ok(content)
    }
}

impl DeepSeekProvider {
    pub async fn new(api_key: String, system_message: String) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = Client::new();
        
        // Verify API key works with a simple test request
        let response = client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", api_key))
            .json(&json!({
                "model": "deepseek-chat",
                "messages": [
                    {
                        "role": "system",
                        "content": "Test message"
                    },
                    {
                        "role": "user",
                        "content": "test"
                    }
                ]
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error = response.text().await?;
            return Err(format!("Failed to initialize DeepSeek provider: {}", error).into());
        }

        Ok(Self {
            api_key,
            client,
            system_message,
        })
    }
}