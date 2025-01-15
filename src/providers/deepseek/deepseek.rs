use reqwest::Client;
use serde_json::json;
use std::error::Error;
use colored::Colorize;
use crate::completion::CompletionProvider;

pub struct DeepSeekProvider {
    api_key: String,
    client: Client,
    personality: PersonalityProfile,
}

impl CompletionProvider for DeepSeekProvider {
    type Error = Box<dyn Error + Send + Sync>;

    async fn complete(&self, prompt: &str) -> Result<String, Self::Error> {
        // Build a character-specific system message
        let mut system_parts = vec![
            format!("You are {}", self.personality.name),
            format!("Role: {}", self.personality.get_str("description").unwrap_or_default()),
            format!("Style: {}", self.personality.get_str("style").unwrap_or_default())
        ];
        
        // Add traits if available
        if let Some(traits) = self.personality.get_array("traits") {
            let trait_list: Vec<_> = traits.iter()
                .filter_map(|v| v.as_str())
                .collect();
            if !trait_list.is_empty() {
                system_parts.push(format!("Traits: {}", trait_list.join(", ")));
            }
        }

        // Add interests/expertise
        if let Some(interests) = self.personality.get_array("interests") {
            let interest_list: Vec<_> = interests.iter()
                .filter_map(|v| v.as_str())
                .collect();
            if !interest_list.is_empty() {
                system_parts.push(format!("Expert in: {}", interest_list.join(", ")));
            }
        }

        // Add communication preferences
        if let Some(prefs) = self.personality.attributes.get("communication_preferences") {
            if let Some(obj) = prefs.as_object() {
                if let Some(style) = obj.get("primary_style") {
                    system_parts.push(format!("Communication style: {}", style.as_str().unwrap_or_default()));
                }
                if let Some(complexity) = obj.get("complexity") {
                    system_parts.push(format!("Technical level: {}", complexity.as_str().unwrap_or_default()));
                }
            }
        }

        let system_message = system_parts.join("\n");
        
        let payload = json!({
            "model": "deepseek-chat",
            "messages": [
                {
                    "role": "system",
                    "content": system_message
                },
                {
                    "role": "user",
                    "content": prompt
                }
            ]
        });

        let response = self.client
            .post("https://api.deepseek.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&payload)
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
    pub async fn new(api_key: String, personality: PersonalityProfile) -> Result<Self, Box<dyn Error + Send + Sync>> {
        Ok(Self {
            api_key,
            client: Client::new(),
            personality,
        })
    }
}