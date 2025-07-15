use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ModelId {
    Auto,
    // OpenAI
    Gpt4o, Gpt4_1, O4Mini, O3, O3Mini,
    // Anthropic
    ClaudeSonnet4, ClaudeOpus4, ClaudeSonnet3_7, ClaudeSonnet3_5, ClaudeHaiku3_5,
    // Google
    Gemini2_0Flash, Gemini2_5Pro,
}

impl Default for ModelId { fn default() -> Self { ModelId::Auto } }

impl ModelId {
    pub fn to_string(&self) -> &'static str {
        match self {
            ModelId::Auto => "Auto (Claude 4 Sonnet)",
            ModelId::Gpt4o => "GPT-4o",
            ModelId::Gpt4_1 => "GPT-4.1",
            ModelId::O4Mini => "o4-mini",
            ModelId::O3 => "o3",
            ModelId::O3Mini => "o3-mini",
            ModelId::ClaudeSonnet4 => "Claude Sonnet 4",
            ModelId::ClaudeOpus4 => "Claude Opus 4",
            ModelId::ClaudeSonnet3_7 => "Claude Sonnet 3.7",
            ModelId::ClaudeSonnet3_5 => "Claude Sonnet 3.5",
            ModelId::ClaudeHaiku3_5 => "Claude Haiku 3.5",
            ModelId::Gemini2_0Flash => "Gemini 2.0 Flash",
            ModelId::Gemini2_5Pro => "Gemini 2.5 Pro",
        }
    }
}
