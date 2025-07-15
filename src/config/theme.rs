use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ColorPalette {
    pub background: String,
    pub foreground: String,
    pub dim_foreground: Option<String>,
    pub bright_foreground: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CursorColors {
    pub text: String,
    pub cursor: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MatchColors {
    pub foreground: String,
    pub background: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SearchColors {
    pub matches: MatchColors,
    pub focused_match: MatchColors,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BarColors {
    pub foreground: String,
    pub background: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct HintColors {
    pub start: BarColors,
    pub end: BarColors,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SelectionColors {
    pub text: String,
    pub background: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AnsiColors {
    pub black: String,
    pub red: String,
    pub green: String,
    pub yellow: String,
    pub blue: String,
    pub magenta: String,
    pub cyan: String,
    pub white: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Colors {
    pub primary: ColorPalette,
    pub cursor: CursorColors,
    pub vi_mode_cursor: Option<CursorColors>,
    pub search: SearchColors,
    pub footer_bar: Option<BarColors>,
    pub hints: Option<HintColors>,
    pub selection: SelectionColors,
    pub normal: AnsiColors,
    pub bright: AnsiColors,
    pub dim: Option<AnsiColors>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Theme {
    pub colors: Colors,
}

pub fn load_theme(path: &Path) -> Result<Theme, Box<dyn std::error::Error>> {
    let theme_str = fs::read_to_string(path)?;
    let theme: Theme = serde_yaml::from_str(&theme_str)?;
    Ok(theme)
}
