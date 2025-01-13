use crate::providers::web_crawler::crawler_manager::WebCrawlerManager;

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
                    println!("\nAnalysis Results:");
                    println!("{:?}", analysis);
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
                    println!("\nResearch Results:");
                    for finding in findings {
                        println!("  {}", finding);
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
                .map(|analysis| {
                    println!("\nLink Analysis:");
                    println!("{}", analysis);
                })
                .map_err(|e| format!("Error following links: {}", e))
        }
        else {
            Err("Unknown web command. Available commands: analyze <url>, research <topic>, links <url>".to_string())
        }
    } else {
        Err("Web crawler is not initialized. Use --crawler flag to enable it.".to_string())
    }
}