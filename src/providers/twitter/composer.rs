use crate::personality::PersonalityProfile;
use crate::providers::twitter::twitbrain::Mention;
use crate::providers::deepseek::DeepSeekProvider;
use crate::completion::CompletionProvider;
use anyhow::Result;

const MAX_TWEET_LENGTH: usize = 280;
const DEFAULT_EMOJI: &str = "💭";

pub struct TweetComposer;

impl TweetComposer {
    async fn get_deepseek_provider(profile: &PersonalityProfile) -> Result<DeepSeekProvider> {
        let api_key = std::env::var("DEEPSEEK_API_KEY")
            .map_err(|_| anyhow::anyhow!("DEEPSEEK_API_KEY environment variable is not set. Please set it to use AI tweet generation."))?;
        
        let examples = if let Some(examples) = profile.attributes.get("example_tweets") {
            if let Some(arr) = examples.as_array() {
                let example_list: Vec<String> = arr.iter()
                    .filter_map(|v| v.as_str())
                    .take(5)
                    .enumerate()
                    .map(|(i, t)| format!("{}. {}", i + 1, t))
                    .collect();
                if !example_list.is_empty() {
                    format!("\nReference tweets (use only as style guide, do not copy):\n{}\n", example_list.join("\n"))
                } else {
                    String::new()
                }
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let interests = if let Some(interests) = profile.attributes.get("interests") {
            if let Some(arr) = interests.as_array() {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            } else {
                String::new()
            }
        } else {
            String::new()
        };
        
        let system_message = format!(
            "You are {} - {}\n\
             Your expertise: {}\n\
             Voice: {}\n\
             \nShare your insights naturally, as if talking to fellow professionals. Mix up your style between:\n\
             - Technical observations\n\
             - Problem-solving stories\n\
             - Industry reflections\n\
             - Tips and best practices\n\
             {}", 
            profile.name,
            profile.get_str("description").unwrap_or("an AI assistant"),
            if interests.is_empty() { "technology, science, innovation" } else { &interests },
            profile.get_str("style").unwrap_or("professional"),
            examples
        );

        DeepSeekProvider::new(api_key, system_message)
            .await
            .map_err(|e| anyhow::anyhow!("Failed to create DeepSeek provider: {}", e))
    }

    // Helper function to count approximate tokens (rough estimation)
    fn count_tokens(text: &str) -> usize {
        // Rough approximation: split on whitespace and punctuation
        text.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .filter(|s| !s.is_empty())
            .count()
    }

    pub async fn generate_auto_post_topic(profile: &PersonalityProfile) -> Result<String> {
        let deepseek = Self::get_deepseek_provider(profile).await?;
        let prompt = "Human: Hey! What's something cool you've been thinking about lately?\nAssistant: I'd like to explore:";

        println!("🔍 Prompt tokens (approx): {}", Self::count_tokens(&prompt));
        let topic = deepseek.complete(&prompt).await?;
        let token_count = Self::count_tokens(&topic);
        println!("📊 Response tokens (approx): {}", token_count);
        
        Ok(Self::truncate_content(topic))
    }

    pub async fn generate_auto_tweet(profile: &PersonalityProfile) -> Result<String> {
        let deepseek = Self::get_deepseek_provider(profile).await?;
        
        let prompt = "Human: Tell me about that! Share your thoughts in a tweet.\nAssistant: Here's what I want to share:";

        println!("🔍 Prompt tokens (approx): {}", Self::count_tokens(&prompt));
        let tweet = deepseek.complete(&prompt).await?;
        let token_count = Self::count_tokens(&tweet);
        println!("📊 Response tokens (approx): {}", token_count);
        
        // Improved content extraction
        let clean_tweet = tweet
            .lines()
            .filter(|line| !line.is_empty())
            .find(|line| 
                !line.starts_with("Human:") && 
                !line.starts_with("Assistant:") &&
                !line.starts_with("Certainly!") &&
                !line.starts_with("Here's") &&
                !line.contains("Let me know") &&
                line.len() > 10
            )
            .unwrap_or(&tweet)
            .trim()
            .trim_matches('"')
            .to_string();

        Ok(Self::truncate_content(clean_tweet))
    }

    pub async fn generate_auto_reply(profile: &PersonalityProfile, original_tweet: &str) -> Result<String> {
        let deepseek = Self::get_deepseek_provider(profile).await?;
        let prompt = format!(
            "As {}, create a thoughtful reply to this tweet: '{}' \
             Maintain your unique voice while adding value to the conversation.",
            profile.name,
            original_tweet
        );
        let reply = deepseek.complete(&prompt).await?;
        Ok(Self::truncate_content(reply))
    }

    pub async fn generate_dm(profile: &PersonalityProfile, recipient: &str) -> Result<String> {
        let deepseek = Self::get_deepseek_provider(profile).await?;
        let prompt = format!(
            "As {}, write a professional direct message to @{}. \
             Keep it friendly yet professional, reflecting your personality.",
            profile.name,
            recipient
        );
        let dm = deepseek.complete(&prompt).await?;
        Ok(Self::truncate_content(dm))
    }

    pub async fn generate_mention_response(profile: &PersonalityProfile, mention: &Mention) -> Result<String> {
        let deepseek = Self::get_deepseek_provider(profile).await?;
        let prompt = format!(
            "As {}, respond to this mention: '{}' \
             Keep your response engaging and authentic to your character.",
            profile.name,
            mention.text
        );
        let response = deepseek.complete(&prompt).await?;
        Ok(Self::truncate_content(response))
    }

    pub async fn generate_auto_post(profile: &PersonalityProfile) -> Result<String> {
        let topic = Self::generate_auto_post_topic(profile).await?;
        println!("📝 Generated topic: \"{}\"", topic);
        
        let deepseek = Self::get_deepseek_provider(profile).await?;
        
        // Get character-specific tweet templates if available
        let tweet_templates = if let Some(templates) = profile.attributes.get("tweet_templates") {
            if let Some(arr) = templates.as_array() {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .take(3)
                    .map(|t| format!("- '{}'", t))
                    .collect::<Vec<_>>()
                    .join("\n")
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // Use character's style for prompt
        let style = profile.get_str("style").unwrap_or("professional");
        let prompt = if !tweet_templates.is_empty() {
            format!(
                "Human: Share your thoughts about '{}' in your {} style.\n\
                 Use one of your signature formats:\n{}\n\
                 Assistant: Here's my update:", 
                topic,
                style,
                tweet_templates
            )
        } else {
            format!(
                "Human: Share your thoughts about '{}' in your {} style. Be creative and authentic to your character.\n\
                 Assistant: Here's my update:", 
                topic,
                style
            )
        };

        println!("🔍 Prompt tokens (approx): {}", Self::count_tokens(&prompt));
        let tweet = deepseek.complete(&prompt).await?;
        let token_count = Self::count_tokens(&tweet);
        println!("📊 Response tokens (approx): {}", token_count);
        
        let clean_tweet = tweet
            .lines()
            .filter(|line| !line.is_empty())
            .find(|line| 
                !line.starts_with("Human:") && 
                !line.starts_with("Assistant:") &&
                !line.starts_with("Certainly!") &&
                !line.starts_with("Here's") &&
                !line.starts_with("Just") &&
                !line.contains("Let me know") &&
                line.len() > 10
            )
            .unwrap_or(&tweet)
            .trim()
            .trim_matches('"')
            .to_string();

        Ok(Self::truncate_content(clean_tweet))
    }

    fn truncate_content(content: String) -> String {
        content.chars().take(MAX_TWEET_LENGTH).collect()
    }
}
