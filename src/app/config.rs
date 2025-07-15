// STUB: Configuration structs will go here.
use serde::{Deserialize, Serialize};

// FIX: Added stub for TextConfig
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextConfig {
    pub font_size: f32,
    pub line_height: f32,
    pub use_ligatures: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub keybindings_path: String,
    pub drive_path: Option<String>,
    pub vim_mode: bool,
    pub openai_api_key: Option<String>,
    pub appearance: Appearance,
    pub text_config: TextConfig,
    pub theme_path: Option<String>, // Add theme_path for theme selection
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appearance {
    pub opacity: f32,
    pub cursor: CursorConfig,
    pub prompt_mode: PromptMode,
    pub warpish_prompt: WarpishPromptConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorConfig {
    pub shape: CursorShape,
    pub blink: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CursorShape { Block, Bar, Underline }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PromptMode { Warpish, Shell }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarpishPromptConfig {
    pub chips: Vec<String>,
    pub same_line: bool,
} 