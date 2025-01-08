use std::env;
use std::io::Write;
use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;
use log::{info, error};

use crate::memory::{ShortTermMemory, LongTermMemory};
use crate::providers::deepseek::DeepSeekProvider;
use crate::completion::CompletionProvider;
use crate::knowledge_base::knowledge_base::KnowledgeBaseHandler;
use crate::database::Database;
use crate::learning::LearningManager;
use crate::personality::{Personality, PersonalityProfile};

mod memory;
mod providers;
mod completion;
mod knowledge_base;
mod database;
mod learning;
mod personality;

// Command-line arguments
#[derive(Parser, Debug)]
struct Args {
    #[arg(short, long)]
    api_key: Option<String>,
}

// Main function
#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::init();
    info!("Starting Rust AI Agent...");

    // Load environment variables
    dotenv().ok();

    // Parse command-line arguments
    let args = Args::parse();

    // Use API key from command line or environment variable
    let api_key = args.api_key
        .or_else(|| env::var("DEEPSEEK_API_KEY").ok())
        .expect("DeepSeek API key not provided")
        .clone(); // Clone the api_key here

    // Initialize memories
    let mut short_term_memory = ShortTermMemory::new();
    let long_term_memory = LongTermMemory::new();

    // Initialize with default personality
    let mut current_personality = Personality::HelpfulAssistant.clone();
    let mut deepseek_provider = DeepSeekProvider::new(api_key.clone(), current_personality.clone());

    // Initialize database
    let database = Database::new("data/agent.db")
        .await
        .expect("Failed to initialize database");

    // Initialize knowledge base handler
    let knowledge_base_handler = KnowledgeBaseHandler::new("data/knowledge_base.json");

    // Initialize learning manager
    let learning_manager = LearningManager::new(database.clone(), knowledge_base_handler.clone());

    // Welcome message with colored output
    println!("{}", "Welcome to the Rust AI Agent!".green());
    println!("Type '{}' to quit.", "exit".red());
    println!("Available personalities:");
    println!("  - Type '{}' for Helpful Assistant", "helpful".cyan());
    println!("  - Type '{}' for Friendly Chat", "friendly".cyan());
    println!("  - Type '{}' for Expert Advisor", "expert".cyan());
    println!("  - Type a personality filename (e.g., 'masterchef_scientist.json') to load a custom personality");

    loop {
        // User input prompt
        print!("{} ", "You:".cyan());
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Exit command
        if input.eq_ignore_ascii_case("exit") {
            println!("{}", "Goodbye!".green());
            // Save long-term memory to file before exiting
            if let Err(e) = long_term_memory.save_to_file("memory.json") {
                error!("Failed to save long-term memory: {}", e);
            }
            break;
        }

        // Check for personality filename loading
        if input.ends_with(".json") {
            if let Some(custom_personality) = load_personality_from_filename(input) {
                current_personality = custom_personality.clone();
                deepseek_provider = DeepSeekProvider::new(api_key.clone(), current_personality.clone());
                println!(
                    "{} {}", 
                    "Loaded custom personality:".blue(),
                    current_personality.to_string().cyan()
                );
                continue;
            }
        }

        // Check for personality switch
        if let Some(new_personality) = Personality::from_input(input) {
            current_personality = new_personality.clone();
            deepseek_provider = DeepSeekProvider::new(api_key.clone(), current_personality.clone());
            println!(
                "{} {}", 
                "Switching personality to:".blue(),
                current_personality.to_string().cyan()
            );
            continue;
        }

        // Process the input and get AI response
        let context = short_term_memory.get_context(&input);
        let prompt = if !context.is_empty() {
            format!(
                "Previous relevant context:\n{}\n\nCurrent conversation:\nUser: {}\n\nRemember that you are acting as: {}", 
                context, 
                input,
                current_personality.system_message()
            )
        } else {
            format!(
                "You are acting as: {}\n\nUser: {}", 
                current_personality.system_message(),
                input
            )
        };

        match deepseek_provider.complete(&prompt).await {
            Ok(response) => {
                let response = response.to_string();
                
                // Store in short-term memory with topic tracking
                short_term_memory.add_interaction(&input, &response);
                
                // Save to database
                if let Err(e) = database.save_conversation(input.to_string(), response.clone(), current_personality.to_string()).await {
                    error!("Failed to save conversation to database: {}", e);
                }

                // Learn from the interaction
                if let Err(e) = learning_manager.learn_from_interaction(&input, &response).await {
                    error!("Failed to learn from interaction: {}", e);
                }

                // Display the response with current personality
                println!("{} [{}] {}", 
                    "Assistant".yellow(),
                    current_personality.to_string().cyan(),
                    response
                );

                // Show memory status if requested
                if input.contains("memory status") {
                    println!("\n{}", short_term_memory.get_memory_stats());
                    
                    if let Ok(summary) = learning_manager.get_learning_summary().await {
                        println!("\n{}", summary);
                    }
                }
            }
            Err(e) => {
                error!("Failed to prompt DeepSeek: {}", e);
                println!("{} {}", "Assistant:".yellow(), "Error: Failed to get response from DeepSeek");
            }
        }
    }
}

use std::fs;

impl std::fmt::Display for Personality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Personality::HelpfulAssistant => write!(f, "Helpful Assistant"),
            Personality::FriendlyChat => write!(f, "Friendly Chat"),
            Personality::ExpertAdvisor => write!(f, "Expert Advisor"),
            Personality::Custom(profile) => write!(f, "{}", profile.name),
        }
    }
}

fn load_personality_from_json(path: &str) -> Option<Personality> {
    match fs::read_to_string(path) {
        Ok(json_str) => {
            match PersonalityProfile::from_json(&json_str) {
                Ok(profile) => Some(Personality::Custom(profile)),
                Err(e) => {
                    error!("Failed to parse personality JSON: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            error!("Failed to read personality JSON file: {}", e);
            None
        }
    }
}

fn load_personality_from_filename(filename: &str) -> Option<Personality> {
    // Define the base directory for characters
    let base_path = "/root/Rust-Ai-Agent---Deepseek/characters/";
    
    // Construct full path
    let full_path = format!("{}{}", base_path, filename);
    
    // Ensure the filename ends with .json
    if !filename.ends_with(".json") {
        error!("Character file must end with .json");
        return None;
    }

    match fs::read_to_string(&full_path) {
        Ok(json_str) => {
            match PersonalityProfile::from_json(&json_str) {
                Ok(profile) => Some(Personality::Custom(profile)),
                Err(e) => {
                    error!("Failed to parse character JSON: {}", e);
                    None
                }
            }
        }
        Err(e) => {
            error!("Failed to read character file '{}': {}", filename, e);
            None
        }
    }
}
