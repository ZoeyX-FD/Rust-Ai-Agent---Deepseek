use crate::providers::web_crawler::crawler_manager::WebCrawlerManager;
use colored::Colorize;

pub async fn handle_command(
    input: &str,
    crawler: &mut Option<WebCrawlerManager>
) -> Result<(), String> {
    if let Some(ref mut crawler) = crawler {
        if input.starts_with("analyze ") {
            let url = input.trim_start_matches("analyze ").trim();
            if url.is_empty() {
                println!("Please provide a URL to analyze.");
                println!("Usage: analyze <url>");
                return Ok(());
            }
            crawler.analyze_webpage(url).await
                .map(|analysis| {
                    for line in analysis {
                        if line.starts_with("🔍") {
                            println!("{}", line.bright_cyan());
                        } else if line.starts_with("📑") {
                            println!("{}", line.bright_yellow());
                        } else if line.starts_with("📝") {
                            println!("{}", line.bright_green());
                        } else if line.starts_with("──") {
                            println!("{}", line.bright_black());
                        } else if !line.is_empty() {
                            println!("  • {}", line);
                        } else {
                            println!();
                        }
                    }
                })
                .map_err(|e| format!("Error analyzing webpage: {}", e))
        }
        else if input.starts_with("research ") {
            let topic = input.trim_start_matches("research ").trim();
            if topic.is_empty() {
                println!("Please provide a topic to research.");
                println!("Usage: research <topic>");
                return Ok(());
            }
            crawler.research_topic(topic).await
                .map(|findings| {
                    for finding in findings {
                        if finding.starts_with("📚") {
                            println!("\n{}", finding.bright_cyan());
                        } else if finding.starts_with("🔍") {
                            println!("\n{}", finding.bright_yellow());
                        } else if finding.starts_with("💡") {
                            println!("\n{}", finding.bright_green());
                        } else if finding.starts_with("📊") {
                            println!("\n{}", finding.bright_cyan());
                        } else if finding.starts_with("──") {
                            println!("{}", finding.bright_black());
                        } else if !finding.is_empty() {
                            println!("  • {}", finding);
                        }
                    }
                })
                .map_err(|e| format!("Error during research: {}", e))
        }
        else if input.starts_with("links ") {
            let url = input.trim_start_matches("links ").trim();
            if url.is_empty() {
                println!("Please provide a URL to extract links from.");
                println!("Usage: links <url>");
                return Ok(());
            }
            crawler.follow_links(url, 1).await
                .map(|result| {
                    for line in result.lines() {
                        if line.starts_with("🔗") {
                            println!("\n{}", line.bright_cyan());
                        } else if line.starts_with("Total") {
                            println!("\n{}", line.bright_yellow());
                        } else if line.starts_with("──") {
                            println!("{}", line.bright_black());
                        } else if !line.is_empty() {
                            println!("  • {}", line);
                        }
                    }
                })
                .map_err(|e| format!("Error following links: {}", e))
        }
        else {
            Err("Unknown web command. Available commands:\n  analyze <url> - Analyze webpage content\n  research <topic> - Research a topic\n  links <url> - Extract links from webpage".to_string())
        }
    } else {
        Err("Web crawler is not initialized. Use --crawler flag to enable it.".to_string())
    }
}