use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::Duration;
use tokio::time;
use url::Url;
use urlencoding;

const DEFAULT_TIMEOUT: u64 = 5;
const MAX_REDIRECTS: usize = 2;
const RATE_LIMIT_DELAY: u64 = 1;
const USER_AGENT: &str = "Mozilla/5.0 (compatible; AIAgent/1.0)";

#[derive(Debug, Clone)]
pub struct WebCrawler {
    client: Client,
    last_visit: std::time::Instant,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PageContent {
    pub url: String,
    pub title: Option<String>,
    pub text: String,
    pub links: Vec<String>,
}

impl WebCrawler {
    pub fn new() -> Result<Self, Box<dyn Error + Send + Sync>> {
        let client = Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT))
            .user_agent(USER_AGENT)
            .redirect(reqwest::redirect::Policy::limited(MAX_REDIRECTS))
            .build()?;

        Ok(Self {
            client,
            last_visit: std::time::Instant::now(),
        })
    }

    async fn rate_limit(&self) {
        let elapsed = self.last_visit.elapsed();
        if elapsed < Duration::from_secs(RATE_LIMIT_DELAY) {
            time::sleep(Duration::from_secs(RATE_LIMIT_DELAY) - elapsed).await;
        }
    }

    pub async fn search(&self, query: &str) -> Result<Vec<String>, Box<dyn Error + Send + Sync>> {
        Ok(vec![
            format!("https://www.google.com/search?q={}", urlencoding::encode(query)),
            format!("https://en.wikipedia.org/wiki/Special:Search?search={}", urlencoding::encode(query)),
            format!("https://duckduckgo.com/?q={}", urlencoding::encode(query)),
        ])
    }

    pub async fn visit_page(&self, url: &str) -> Result<PageContent, Box<dyn Error + Send + Sync>> {
        self.rate_limit().await;

        let response = self.client
            .get(url)
            .send()
            .await?;

        let final_url = response.url().to_string();
        let html = response.text().await?;
        
        // Extract title
        let title = html
            .split("<title>")
            .nth(1)
            .and_then(|s| s.split("</title>").next())
            .map(|s| s.to_string());

        // Extract text content
        let text = html
            .split('>')
            .filter_map(|part| {
                let content = part.split('<').next()?;
                if !content.trim().is_empty() {
                    Some(content.trim())
                } else {
                    None
                }
            })
            .collect::<Vec<&str>>()
            .join(" ");

        // Extract links
        let links: Vec<String> = html
            .split("href=\"")
            .skip(1)
            .filter_map(|part| {
                let url = part.split('"').next()?;
                if url.starts_with("http") {
                    Some(url.to_string())
                } else {
                    None
                }
            })
            .take(20)
            .collect();

        Ok(PageContent {
            url: final_url,
            title,
            text,
            links,
        })
    }
}
