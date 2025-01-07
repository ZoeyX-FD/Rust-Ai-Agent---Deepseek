use clap::Parser;
use std::env;
use std::io::{self, Write};
use log::{info, error};
use dotenv::dotenv;
use crate::memory::{ShortTermMemory, LongTermMemory};
use crate::providers::deepseek::DeepSeekProvider;
use crate::completion::CompletionProvider;

mod memory;
mod providers;
mod completion;

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

    // Create DeepSeek provider
    let deepseek_provider = DeepSeekProvider::new(deepseek_api_key);

    // Initialize memory
    let mut short_term_memory = ShortTermMemory::new(10); // Store last 10 messages
    let mut long_term_memory = LongTermMemory::new();

    // Load long-term memory from file (if exists)
    if let Ok(memory) = LongTermMemory::load_from_file("memory.json") {
        long_term_memory = memory;
        info!("Loaded long-term memory from file.");
    }

    println!("Welcome to the Rust AI Agent! Type 'exit' to quit.");

    loop {
        print!("You: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("exit") {
            println!("Goodbye!");
            // Save long-term memory to file before exiting
            if let Err(e) = long_term_memory.save_to_file("memory.json") {
                error!("Failed to save long-term memory: {}", e);
            }
            break;
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

        // Prepare context for DeepSeek
        let context = format!(
            "{}\n{}",
            long_term_memory.retrieve("conversation_summary").unwrap_or(&String::new()),
            short_term_memory.get_context()
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

        println!("AI: {}", response);
    }
}
