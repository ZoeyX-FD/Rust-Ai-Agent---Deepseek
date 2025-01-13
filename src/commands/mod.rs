use std::io::Write;
use colored::Colorize;

use crate::providers::deepseek::DeepSeekProvider;
use crate::personality::PersonalityProfile;
use crate::providers::twitter::manager::ConversationManager;
use crate::providers::web_crawler::crawler_manager::WebCrawlerManager;
use crate::completion::CompletionProvider;

mod character;
mod twitter;
mod web;
mod system;

pub struct CommandHandler {
    twitter_manager: Option<ConversationManager>,
    web_crawler: Option<WebCrawlerManager>,
    deepseek_provider: DeepSeekProvider,
    personality: PersonalityProfile,
}

impl CommandHandler {
    pub fn new(
        twitter_manager: Option<ConversationManager>,
        web_crawler: Option<WebCrawlerManager>,
        deepseek_provider: DeepSeekProvider,
        personality: PersonalityProfile,
    ) -> Self {
        Self {
            twitter_manager,
            web_crawler,
            deepseek_provider,
            personality,
        }
    }

    pub async fn handle_command(&mut self, input: &str) -> Result<(), String> {
        if input.is_empty() {
            return Ok(());
        }

        let input = input.trim();

        // Handle single-word commands first
        match input.to_lowercase().as_str() {
            "help" => return self.handle_system_command(input).await,
            "exit" | "quit" => return self.handle_system_command(input).await,
            "chars" | "characters" => return self.handle_character_command(input).await,
            "load" => return self.handle_character_command(input).await,
            "" => return Ok(()),
            _ => {}
        }

        // Handle command prefixes
        if input.starts_with("load ") {
            return self.handle_character_command(input).await;
        }

        // Twitter commands
        if input.starts_with("tweet ") || 
           input.starts_with("autopost ") || 
           input.eq_ignore_ascii_case("tweet") ||
           input.eq_ignore_ascii_case("autopost") ||
           input.starts_with("reply ") || 
           input.starts_with("dm @") {
            return self.handle_twitter_command(input).await;
        }

        // Web commands
        if input.starts_with("analyze ") || 
           input.eq_ignore_ascii_case("analyze") ||
           input.starts_with("research ") ||
           input.eq_ignore_ascii_case("research") ||
           input.starts_with("links ") ||
           input.eq_ignore_ascii_case("links") {
            return self.handle_web_command(input).await;
        }

        // Default to chat completion if no command matches
        self.handle_chat_command(input).await
    }

    async fn handle_twitter_command(&mut self, input: &str) -> Result<(), String> {
        if input.eq_ignore_ascii_case("tweet") {
            println!("Please provide a message to tweet.");
            println!("Usage: tweet <message>");
            return Ok(());
        }
        if input.eq_ignore_ascii_case("autopost") {
            println!("Please specify start or stop for autopost.");
            println!("Usage: autopost start <minutes> or autopost stop");
            return Ok(());
        }
        twitter::handle_command(input, &mut self.twitter_manager).await
    }

    async fn handle_web_command(&mut self, input: &str) -> Result<(), String> {
        match input.to_lowercase().as_str() {
            "analyze" => {
                println!("Please provide a URL to analyze.");
                println!("Usage: analyze <url>");
                Ok(())
            }
            "research" => {
                println!("Please provide a topic to research.");
                println!("Usage: research <topic>");
                Ok(())
            }
            "links" => {
                println!("Please provide a URL to extract links from.");
                println!("Usage: links <url>");
                Ok(())
            }
            _ => web::handle_command(input, &mut self.web_crawler).await
        }
    }

    async fn handle_character_command(&mut self, input: &str) -> Result<(), String> {
        let result = character::handle_command(input, &mut self.personality);
        if result.is_ok() {
            // Update DeepSeek provider with new personality
            if let Err(e) = self.deepseek_provider.update_personality(
                self.personality.generate_system_prompt()
            ).await {
                return Err(format!("Failed to update personality: {}", e));
            }
        }
        result
    }

    async fn handle_system_command(&mut self, input: &str) -> Result<(), String> {
        system::handle_command(input)
    }

    async fn handle_chat_command(&mut self, input: &str) -> Result<(), String> {
        // Estimate input tokens (rough approximation: 4 chars = 1 token)
        let input_tokens = (input.chars().count() as f32 / 4.0).ceil() as usize;
        
        match self.deepseek_provider.complete(input).await {
            Ok(response) => {
                // Estimate response tokens
                let response_tokens = (response.chars().count() as f32 / 4.0).ceil() as usize;
                
                self.print_response(
                    &self.personality.name,
                    &response,
                    input_tokens,
                    response_tokens
                );
                Ok(())
            }
            Err(e) => Err(format!("Error: {}", e)),
        }
    }

    fn print_response(&self, character_name: &str, response: &str, input_tokens: usize, response_tokens: usize) {
        println!("\n🤖 [{}]", character_name.to_string().bright_yellow());
        println!("{}", response);
        println!("\n📊 Tokens: {} in / {} out (total: {})", 
            input_tokens.to_string().cyan(),
            response_tokens.to_string().cyan(),
            (input_tokens + response_tokens).to_string().cyan()
        );
        println!();
    }
}