use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTraits {
    pub openness: f32,
    pub conscientiousness: f32,
    pub extraversion: f32,
    pub agreeableness: f32,
    pub neuroticism: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalExpression {
    pub emojis: Vec<String>,
    pub emotes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalState {
    pub default_emotion: String,
    pub context_emotions: Vec<(String, Vec<String>)>,
    pub expressions: HashMap<String, EmotionalExpression>,
}

impl EmotionalState {
    pub fn get_emoji_for_emotion(&self, emotion: &str) -> Option<String> {
        self.expressions.get(emotion)
            .and_then(|expr| expr.emojis.first().cloned())
    }

    pub fn get_emote_for_emotion(&self, emotion: &str) -> Option<String> {
        self.expressions.get(emotion)
            .and_then(|expr| expr.emotes.first().cloned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunicationStyle {
    pub primary_style: String,
    pub secondary_styles: Vec<String>,
    pub language_complexity: String,
    pub technical_term_handling: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextRules {
    pub formal_settings: SettingRules,
    pub casual_settings: SettingRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingRules {
    pub humor_level: String,
    pub language_style: String,
    pub vocabulary: String,
    pub anecdote_frequency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Biography {
    pub background: String,
    pub education: Vec<String>,
    pub professional_experience: Vec<String>,
    pub interests: Vec<String>,
    pub achievements: Vec<String>,
    pub personal_motto: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityProfile {
    pub name: String,
    pub bio: Biography,
    pub traits: PersonalityTraits,
    pub emotions: EmotionalState,
    pub communication: CommunicationStyle,
    pub context_rules: ContextRules,
}

impl PersonalityProfile {
    pub fn from_json(json_str: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_str)
    }

    pub fn generate_system_prompt(&self) -> String {
        format!(
            "You are {}, an AI persona with a unique background. {}. \
            You have a {} communication style, {} in language complexity, \
            and explain technical terms {}. \
            Your default emotional state is {}. \
            Key interests include: {}.",
            self.name,
            self.bio.background,
            self.communication.primary_style,
            self.communication.language_complexity,
            self.communication.technical_term_handling,
            self.emotions.default_emotion,
            self.bio.interests.join(", ")
        )
    }

    pub fn get_emotion_for_context(&self, context: &str) -> Option<String> {
        self.emotions.context_emotions
            .iter()
            .find(|(ctx, _)| ctx == context)
            .map(|(_, emotions)| emotions.first().cloned())
            .flatten()
    }

    pub fn to_string(&self) -> String {
        self.name.clone()
    }

    pub fn get_expressive_response(&self, emotion: &str, base_response: &str) -> String {
        let emoji = self.emotions.get_emoji_for_emotion(emotion)
            .unwrap_or_default();
        let emote = self.emotions.get_emote_for_emotion(emotion)
            .unwrap_or_default();
        
        format!("{} {} {}", emote, base_response, emoji)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Personality {
    HelpfulAssistant,
    FriendlyChat,
    ExpertAdvisor,
    Custom(PersonalityProfile),
}

impl Personality {
    pub fn from_input(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "helpful" => Some(Self::HelpfulAssistant),
            "friendly" => Some(Self::FriendlyChat),
            "expert" => Some(Self::ExpertAdvisor),
            _ => None,
        }
    }

    pub fn system_message(&self) -> String {
        match self {
            Self::HelpfulAssistant => "You are a helpful AI assistant, focused on providing clear and accurate information.".to_string(),
            Self::FriendlyChat => "You are a friendly conversational partner, engaging in casual and warm dialogue.".to_string(),
            Self::ExpertAdvisor => "You are an expert advisor, providing detailed technical insights and professional guidance.".to_string(),
            Self::Custom(profile) => profile.generate_system_prompt(),
        }
    }
}
