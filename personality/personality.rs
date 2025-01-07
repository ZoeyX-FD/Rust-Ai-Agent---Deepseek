   // personality/mod.rs
   #[derive(Clone, Debug)]
   pub enum Personality {
    HelpfulAssistant,
    FriendlyCompanion,
    ExpertAdvisor,
}

impl Personality {
    pub fn system_message(&self) -> &str {
        match self {
            Personality::HelpfulAssistant => "You are a helpful assistant.",
            Personality::FriendlyCompanion => "You are a friendly companion.",
            Personality::ExpertAdvisor => "You are an expert advisor.",
        }
    }

    pub fn from_input(input: &str) -> Option<Self> {
        match input.to_lowercase().as_str() {
            "helpful" => Some(Personality::HelpfulAssistant),
            "friendly" => Some(Personality::FriendlyCompanion),
            "expert" => Some(Personality::ExpertAdvisor),
            _ => None,
        }
    }
}