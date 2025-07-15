pub mod theme;

use crate::agent::model::ModelId;
use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AiConfig {
    #[serde(default)]
    pub base_model: ModelId,
    #[serde(default)]
    pub planning_model: ModelId,
    #[serde(default = "default_ollama_url")]
    pub ollama_url: String,
    #[serde(default = "default_ollama_model")]
    pub ollama_model: String,
    #[serde(default = "default_true")]
    pub enable_ai_completions: bool,
    #[serde(default = "default_ai_timeout")]
    pub ai_timeout_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CompletionsConfig {
    #[serde(default = "default_true")]
    pub enabled: bool,
    #[serde(default = "default_trigger_chars")]
    pub trigger_chars: Vec<char>,
    #[serde(default = "default_min_trigger_length")]
    pub min_trigger_length: usize,
    #[serde(default = "default_max_suggestions")]
    pub max_suggestions: usize,
    #[serde(default = "default_true")]
    pub show_descriptions: bool,
    #[serde(default = "default_true")]
    pub show_type_indicators: bool,
    #[serde(default = "default_cache_duration")]
    pub cache_duration_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EditorConfig {
    #[serde(default = "default_true")]
    pub autosuggestions: bool,
    #[serde(default = "default_true")]
    pub autocomplete_pairs: bool,
    #[serde(default = "default_true")]
    pub copy_on_select: bool,
    #[serde(default = "default_true")]
    pub vim_enabled: bool,
    #[serde(default = "default_completions")]
    pub completions: CompletionsConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppearanceConfig {
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default = "default_line_height")]
    pub line_height: f32,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_font_weight")]
    pub font_weight: String,
    #[serde(default = "default_true")]
    pub use_ligatures: bool,
    #[serde(default = "default_opacity")]
    pub opacity: f32,
    #[serde(default = "default_true")]
    pub blur: bool,
    #[serde(default = "default_window_size")]
    pub window_size: WindowSizeConfig,
    #[serde(default = "default_prompt_mode")]
    pub prompt_mode: PromptMode,
    #[serde(default = "default_input_position")]
    pub input_position: InputPosition,
    #[serde(default = "default_cursor")]
    pub cursor: CursorConfig,
    #[serde(default = "default_warpish_prompt")]
    pub warpish_prompt: WarpishPromptConfig,
    #[serde(default = "default_theme")]
    pub theme: ThemeConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ThemeConfig {
    #[serde(default = "default_theme_name")]
    pub name: String,
    #[serde(default = "default_true")]
    pub sync_with_os: bool,
    #[serde(default = "default_os_theme_mode")]
    pub os_theme_mode: OsThemeMode,
    #[serde(default = "default_theme_path")]
    pub custom_theme_path: Option<String>,
    #[serde(default = "default_true")]
    pub auto_preview: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum OsThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WindowSizeConfig {
    #[serde(default = "default_use_custom_size")]
    pub use_custom_size: bool,
    #[serde(default = "default_columns")]
    pub columns: u16,
    #[serde(default = "default_rows")]
    pub rows: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CursorConfig {
    #[serde(default = "default_cursor_shape")]
    pub shape: CursorShape,
    #[serde(default = "default_true")]
    pub blink: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct WarpishPromptConfig {
    #[serde(default = "default_prompt_chips")]
    pub chips: Vec<String>,
    #[serde(default = "default_true")]
    pub same_line: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum PromptMode {
    #[default]
    Simple,
    Warpish,
    None,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum InputPosition {
    #[default]
    Bottom,
    Top,
    Floating,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum CursorShape {
    #[default]
    Block,
    Underline,
    Beam,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub prompt: Option<String>,
    pub ai_api_key: Option<String>,
    #[serde(default)]
    pub ai: AiConfig,
    #[serde(default)]
    pub editor: EditorConfig,
    #[serde(default)]
    pub appearance: AppearanceConfig,
    pub user: Option<UserConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserConfig {
    pub shell: Option<String>,
    pub home_dir: Option<PathBuf>,
}

// Default value functions
fn default_true() -> bool { true }
fn default_font_size() -> f32 { 14.0 }
fn default_line_height() -> f32 { 1.2 }
fn default_font_family() -> String { "JetBrains Mono".to_string() }
fn default_font_weight() -> String { "Regular".to_string() }
fn default_opacity() -> f32 { 1.0 }
fn default_ollama_url() -> String { "http://localhost:11434/api/generate".to_string() }
fn default_ollama_model() -> String { "codellama".to_string() }
fn default_ai_timeout() -> u64 { 5 }
fn default_trigger_chars() -> Vec<char> { vec![' ', '\t', '/', '-', '.'] }
fn default_min_trigger_length() -> usize { 1 }
fn default_max_suggestions() -> usize { 15 }
fn default_cache_duration() -> u64 { 30 }
fn default_completions() -> CompletionsConfig { CompletionsConfig::default() }
fn default_use_custom_size() -> bool { false }
fn default_columns() -> u16 { 80 }
fn default_rows() -> u16 { 24 }
fn default_cursor_shape() -> CursorShape { CursorShape::Block }
fn default_prompt_mode() -> PromptMode { PromptMode::Simple }
fn default_input_position() -> InputPosition { InputPosition::Bottom }
fn default_cursor() -> CursorConfig { CursorConfig::default() }
fn default_warpish_prompt() -> WarpishPromptConfig { WarpishPromptConfig::default() }
fn default_prompt_chips() -> Vec<String> { vec!["cwd".to_string(), "git".to_string(), "time".to_string()] }
fn default_theme() -> ThemeConfig { ThemeConfig::default() }
fn default_theme_name() -> String { "default".to_string() }
fn default_os_theme_mode() -> OsThemeMode { OsThemeMode::System }
fn default_theme_path() -> Option<String> { None }

fn default_window_size() -> WindowSizeConfig { WindowSizeConfig::default() }

// Type alias for compatibility with existing code
pub type TextConfig = AppearanceConfig;

pub fn load_config() -> Result<Config, AppError> {
    let config_str = fs::read_to_string("terminal.toml")
        .map_err(|e| AppError::Config(format!("Failed to read terminal.toml: {}", e)))?;
    
    let mut config: Config = toml::from_str(&config_str)
        .map_err(|e| AppError::Config(format!("Failed to parse terminal.toml: {}", e)))?;

    if config.ai_api_key.is_none() {
        config.ai_api_key = std::env::var("AI_API_KEY").ok();
    }
    
    println!("Configuration file 'terminal.toml' loaded.");

    Ok(config)
}