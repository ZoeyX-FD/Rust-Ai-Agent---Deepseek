use std::fmt;

#[derive(Debug, Clone)]
pub enum Personality {
    HelpfulAssistant,
    FriendlyAssistant,
    ExpertAssistant,
}

impl fmt::Display for Personality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Personality::HelpfulAssistant => write!(f, "Helpful Assistant"),
            Personality::FriendlyAssistant => write!(f, "Friendly Assistant"),
            Personality::ExpertAssistant => write!(f, "Expert Assistant"),
        }
    }
}

impl Personality {
    /// Convert input string to Personality
    pub fn from_input(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "helpful" => Some(Personality::HelpfulAssistant),
            "friendly" => Some(Personality::FriendlyAssistant),
            "expert" => Some(Personality::ExpertAssistant),
            _ => None,
        }
    }

    /// Get the system message for the personality
    pub fn system_message(&self) -> String {
        match self {
            Personality::HelpfulAssistant => "You are a helpful assistant. Provide clear and concise answers.".to_string(),
            Personality::FriendlyAssistant => "You are a friendly assistant. Be warm and engaging in your responses.".to_string(),
            Personality::ExpertAssistant => "You are an expert assistant. Provide detailed and accurate information.".to_string(),
        }
    }
}
