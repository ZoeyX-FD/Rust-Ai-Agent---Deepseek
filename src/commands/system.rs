use colored::Colorize;

pub fn handle_command(input: &str) -> Result<(), String> {
    match input.to_lowercase().as_str() {
        "help" => {
            println!("\n🤖 Available Commands:");
            println!("\nAI Assistant Commands:");
            println!("  Just type your question or request");
            println!("  Examples:");
            println!("    - show me how to create a web server in rust");
            println!("    - explain error handling in rust");
            println!("    - help me debug this code: [your code]");
            
            println!("\nCharacter Commands:");
            println!("  chars         - List available characters");
            println!("  load <name>   - Switch to a different character");
            println!("  Example: load helpful, load friendly");

            println!("\nTwitter Commands:");
            println!("  tweet <message>           - Post a tweet");
            println!("  tweet                     - Generate AI tweet");
            println!("  reply <id> <message>      - Reply to a tweet");
            println!("  dm @user: <message>       - Send a direct message");
            println!("  autopost start <minutes>  - Start auto-posting");
            println!("  autopost stop             - Stop auto-posting");
            println!("  logs                      - Show recent activity");

            println!("\nWeb Commands:");
            println!("  analyze <url>    - Analyze webpage content");
            println!("  research <topic>  - Research a topic");
            println!("  links <url>       - Extract links from webpage");

            println!("\nSystem Commands:");
            println!("  help  - Show this help menu");
            println!("  exit  - Exit the program");
            
            Ok(())
        },
        "exit" | "quit" => {
            println!("👋 Goodbye!");
            std::process::exit(0);
        },
        _ => Err("Unknown system command".to_string()),
    }
}