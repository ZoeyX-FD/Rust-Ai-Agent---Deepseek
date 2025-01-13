use std::env;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use clap::Parser;
use colored::Colorize;
use dotenv::dotenv;

use crate::providers::deepseek::DeepSeekProvider;
use crate::knowledge_base::knowledge_base::KnowledgeBaseHandler;
use crate::database::Database;
use crate::learning::LearningManager;
use crate::personality::{Personality, PersonalityProfile};

// Twitter integration
use crate::providers::twitter::manager::ConversationManager;

// Web crawler integration
use crate::providers::web_crawler::crawler_manager::WebCrawlerManager;

// Command handling
use crate::commands::CommandHandler;

// Module imports
mod memory;
mod providers;
mod knowledge_base;
mod database;
mod learning;
mod completion;
mod personality;
mod commands;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    api_key: Option<String>,

    #[arg(long)]
    twitter: bool,

    #[arg(long)]
    crawler: bool,

    #[arg(long)]
    character: Option<String>,

    #[arg(long)]
    twitter_cookie: Option<String>,

    #[arg(long)]
    twitter_username: Option<String>,

    #[arg(long)]
    twitter_password: Option<String>,

    #[arg(long)]
    twitter_email: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize colored output
    colored::control::set_override(true);

    // Load environment variables
    dotenv().ok();

    // Parse command line arguments
    let args = Args::parse();

    // Get API key from command line or environment
    let api_key = match args.api_key {
        Some(key) => key,
        None => env::var("DEEPSEEK_API_KEY").expect("API key must be provided via --api-key or DEEPSEEK_API_KEY env var"),
    };

    // Initialize personality
    let mut current_personality = if let Some(character_file) = args.character {
        match load_personality_from_filename(&character_file) {
            Some(personality) => personality,
            None => {
                println!("Failed to load character: {}", character_file);
                create_default_personality()
            }
        }
    } else {
        create_default_personality()
    };

    // Extract PersonalityProfile from Personality
    let personality_profile = match &current_personality {
        Personality::Dynamic(profile) => profile.clone(),
    };

    // Initialize Deepseek provider
    let deepseek_provider = DeepSeekProvider::new(
        api_key.clone(),
        personality_profile.generate_system_prompt(),
    ).await?;

    // Initialize database
    let database = Database::new("data/agent.db").await?;

    // Initialize knowledge base handler
    let knowledge_base_handler = KnowledgeBaseHandler::new("data/knowledge_base.json");

    // Initialize learning manager
    let learning_manager = LearningManager::new(database.clone(), knowledge_base_handler.clone());

    // Initialize Twitter integration if enabled
    let mut twitter_manager = if args.twitter {
        println!("🐦 Initializing Twitter integration...");
        match ConversationManager::new(personality_profile.clone()).await {
            Ok(manager) => {
                println!("✅ Twitter integration initialized successfully!");
                Some(manager)
            },
            Err(e) => {
                println!("❌ Failed to initialize Twitter: {}", e);
                None
            }
        }
    } else {
        None
    };

    // Initialize web crawler if enabled
    let mut web_crawler = if args.crawler || env::var("ENABLE_CRAWLER").map(|val| val == "true").unwrap_or(false) {
        Some(WebCrawlerManager::new(personality_profile.clone()).await?)
    } else {
        None
    };

    // Create command handler
    let mut command_handler = CommandHandler::new(
        twitter_manager,
        web_crawler,
        deepseek_provider,
        personality_profile,
    );

    // Show initial help menu
    command_handler.handle_command("help").await?;

    // Main input loop
    loop {
        print!("👤 ");
        std::io::stdout().flush().unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if let Err(e) = command_handler.handle_command(input).await {
            println!("{}", e.red());
        }
    }
}

fn load_personality_from_filename(filename: &str) -> Option<Personality> {
    let path = Path::new("characters").join(filename);
    if path.exists() {
        if let Ok(file) = File::open(path) {
            if let Ok(profile) = serde_json::from_reader::<_, PersonalityProfile>(file) {
                return Some(Personality::Dynamic(profile));
            }
        }
    }
    None
}

fn create_default_personality() -> Personality {
    Personality::Dynamic(PersonalityProfile {
        name: "Helpful Assistant".to_string(),
        attributes: serde_json::json!({
            "description": "a helpful AI coding assistant",
            "style": "professional and technically precise",
            "expertise": "programming, software development, and technical problem-solving",
            "motto": "Always here to help with your coding needs",
            "example_code": [
                "```python\n# Example function\ndef greet(name):\n    return f'Hello, {name}!'\n```",
                "```rust\n// Example struct\nstruct User {\n    name: String,\n    age: u32\n}\n```"
            ]
        }),
    })
}
