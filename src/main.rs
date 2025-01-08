use clap::Parser;
use std::env;
use std::io::{self, Write};
use log::{info, error};
use dotenv::dotenv;
use colored::*;
use crate::memory::{ShortTermMemory, LongTermMemory};
use crate::providers::deepseek::DeepSeekProvider;
use crate::completion::CompletionProvider;
use crate::knowledge_base::knowledge_base::KnowledgeBaseHandler;

// Include the personality module from the root directory
#[path = "../personality/personality.rs"]
mod personality;
use personality::Personality;

mod memory;
mod providers;
mod completion;
mod knowledge_base;

// Command-line arguments
#[derive(Parser, Debug)]
#[command(name = "rust-ai-agent")]
#[command(about = "A Rust-based AI agent using DeepSeek")]
struct Args {
    /// DeepSeek API key
    #[arg(short, long)]
    api_key: Option<String>,
    /// Model to use (e.g., deepseek-chat)
    #[arg(short, long, default_value = "deepseek-chat")]
    model: String,
    /// Temperature for response generation (0.0 to 1.0)
    #[arg(short, long, default_value_t = 0.7)]
    temperature: f32,
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
    let deepseek_api_key = args.api_key
        .or_else(|| env::var("DEEPSEEK_API_KEY").ok())
        .expect("DeepSeek API key not provided");

    // Initialize with default personality
    let mut current_personality = Personality::HelpfulAssistant;
    let mut deepseek_provider = DeepSeekProvider::new(deepseek_api_key.clone(), current_personality.clone());

    // Initialize memory
    let mut short_term_memory = ShortTermMemory::new(10); // Store last 10 messages
    let mut long_term_memory = LongTermMemory::new();

    // Load long-term memory from file (if exists)
    if let Ok(memory) = LongTermMemory::load_from_file("memory.json") {
        long_term_memory = memory;
        info!("Loaded long-term memory from file.");
    }

    // Initialize the knowledge base handler
    let knowledge_base_handler = KnowledgeBaseHandler::new("data/knowledge_base.json");

    // Welcome message with colored output
    println!("{}", "Welcome to the Rust AI Agent!".bright_green());
    println!("Type '{}' to quit.", "exit".bright_red());
    println!("You can change personality by typing '{}', '{}', or '{}'.", 
        "helpful".bright_blue(), 
        "friendly".bright_blue(), 
        "expert".bright_blue());

    loop {
        // User input prompt
        print!("{} ", "You:".bright_cyan());
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        // Exit command
        if input.eq_ignore_ascii_case("exit") {
            println!("{}", "Goodbye!".bright_green());
            // Save long-term memory to file before exiting
            if let Err(e) = long_term_memory.save_to_file("memory.json") {
                error!("Failed to save long-term memory: {}", e);
            }
            break;
        }

        // Personality switching
        if let Some(new_personality) = Personality::from_input(input) {
            println!(
                "{} {}", 
                "Switching personality to:".bright_blue(), 
                new_personality.to_string().bright_yellow()
            );
            current_personality = new_personality.clone();
            deepseek_provider = DeepSeekProvider::new(deepseek_api_key.clone(), current_personality.clone());
            continue;
        }

        // Add user input to short-term memory
        short_term_memory.add_message(format!("User: {}", input));

        // Summarize and compress if necessary
        if short_term_memory.get_context().lines().count() >= 10 {
            match short_term_memory.summarize(&deepseek_provider).await {
                Ok(summary) => {
                    long_term_memory.store("conversation_summary".to_string(), summary);
                    short_term_memory.compress();
                    info!("Conversation summarized and compressed.");
                }
                Err(e) => {
                    error!("Failed to summarize conversation: {}", e);
                }
            }
        }

        // Retrieve relevant information based on user input
        let retrieved_info = knowledge_base_handler.retrieve_information(input);

        // Prepare context for DeepSeek
        let context = format!(
            "{}\n{}\n{}",
            long_term_memory.retrieve("conversation_summary").unwrap_or(&String::new()),
            short_term_memory.get_context(),
            retrieved_info
        );

        // Send input and context to DeepSeek
        let response = deepseek_provider
            .complete(&context)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to prompt DeepSeek: {}", e);
                "Error: Failed to get response from DeepSeek".to_string()
            });

        // Add AI response to short-term memory
        short_term_memory.add_message(format!("AI: {}", response));

        // Print AI response with colored output
        println!("{} {}", "AI:".bright_magenta(), response.bright_yellow());
    }
}
