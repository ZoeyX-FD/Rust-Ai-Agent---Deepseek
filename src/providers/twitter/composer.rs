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
        let mut prompt_parts = vec![
            format!("You are {}", profile.name),
            format!("Role: {}", profile.get_str("description").unwrap_or_default()),
            format!("Style: {}", profile.get_str("style").unwrap_or_default())
        ];
        
        // Add traits if available
        if let Some(traits) = profile.get_array("traits") {
            let trait_list: Vec<_> = traits.iter()
                .filter_map(|v| v.as_str())
                .collect();
            if !trait_list.is_empty() {
                prompt_parts.push(format!("Traits: {}", trait_list.join(", ")));
            }
        }

        // Add interests/expertise
        if let Some(interests) = profile.get_array("interests") {
            let interest_list: Vec<_> = interests.iter()
                .filter_map(|v| v.as_str())
                .collect();
            if !interest_list.is_empty() {
                prompt_parts.push(format!("Expert in: {}", interest_list.join(", ")));
            }
        }

        // Add communication preferences
        if let Some(prefs) = profile.attributes.get("communication_preferences") {
            if let Some(obj) = prefs.as_object() {
                if let Some(style) = obj.get("primary_style") {
                    prompt_parts.push(format!("Communication style: {}", style.as_str().unwrap_or_default()));
                }
                if let Some(complexity) = obj.get("complexity") {
                    prompt_parts.push(format!("Technical level: {}", complexity.as_str().unwrap_or_default()));
                }
            }
        }

        prompt_parts.push("\nTask: Generate a topic for a tweet that aligns with your expertise and interests. The topic should be something you would genuinely want to discuss given your background and personality.\n\nTopic:".to_string());

        let prompt = prompt_parts.join("\n");
        
        let provider = Self::get_deepseek_provider(profile).await?;
        let topic = provider.complete(&prompt).await?;
        
        // Clean up the topic
        let topic = topic.trim()
            .trim_start_matches("Topic:")
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .trim();
        
        Ok(topic.to_string())
    }

    pub async fn generate_auto_tweet(profile: &PersonalityProfile) -> Result<String> {
        let topic = Self::generate_auto_post_topic(profile).await?;
        
        let mut prompt_parts = vec![
            format!("You are {}", profile.name),
            format!("Role: {}", profile.get_str("description").unwrap_or_default()),
            format!("Style: {}", profile.get_str("style").unwrap_or_default())
        ];
        
        // Add traits if available
        if let Some(traits) = profile.get_array("traits") {
            let trait_list: Vec<_> = traits.iter()
                .filter_map(|v| v.as_str())
                .collect();
            if !trait_list.is_empty() {
                prompt_parts.push(format!("Traits: {}", trait_list.join(", ")));
            }
        }

        // Add interests/expertise
        if let Some(interests) = profile.get_array("interests") {
            let interest_list: Vec<_> = interests.iter()
                .filter_map(|v| v.as_str())
                .collect();
            if !interest_list.is_empty() {
                prompt_parts.push(format!("Expert in: {}", interest_list.join(", ")));
            }
        }

        // Add communication preferences
        if let Some(prefs) = profile.attributes.get("communication_preferences") {
            if let Some(obj) = prefs.as_object() {
                if let Some(style) = obj.get("primary_style") {
                    prompt_parts.push(format!("Communication style: {}", style.as_str().unwrap_or_default()));
                }
                if let Some(complexity) = obj.get("complexity") {
                    prompt_parts.push(format!("Technical level: {}", complexity.as_str().unwrap_or_default()));
                }
            }
        }

        // Add example tweets if available
        if let Some(examples) = profile.get_array("example_tweets") {
            let example_list: Vec<_> = examples.iter()
                .filter_map(|v| v.as_str())
                .take(3)  // Limit to 3 examples
                .collect();
            if !example_list.is_empty() {
                prompt_parts.push(format!("\nExample tweets:\n{}", example_list.join("\n")));
            }
        }

        prompt_parts.push(format!("\nTask: Write a tweet about the following topic in your unique voice: \"{}\"\n\nThe tweet should reflect your expertise level and personality. Keep it under 280 characters.\n\nTweet:", topic));

        let prompt = prompt_parts.join("\n");
        
        let provider = Self::get_deepseek_provider(profile).await?;
        let tweet = provider.complete(&prompt).await?;
        
        // Clean up the tweet
        let tweet = tweet.trim()
            .trim_start_matches("Tweet:")
            .trim_start_matches("\"")
            .trim_end_matches("\"")
            .trim();
        
        Ok(tweet.to_string())
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

    fn truncate_content(content: String) -> String {
        content.chars().take(MAX_TWEET_LENGTH).collect()
    }
}
