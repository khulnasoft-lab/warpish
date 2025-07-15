use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomColor(pub u8, pub u8, pub u8);

impl<'de> Deserialize<'de> for CustomColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        let bytes = hex::decode(s.trim_start_matches('#')).map_err(serde::de::Error::custom)?;
        if bytes.len() != 3 {
            return Err(serde::de::Error::custom("Hex color must be 6 digits"));
        }
        Ok(CustomColor(bytes[0], bytes[1], bytes[2]))
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AnsiColors {
    pub black: CustomColor,
    pub red: CustomColor,
    pub green: CustomColor,
    pub yellow: CustomColor,
    pub blue: CustomColor,
    pub magenta: CustomColor,
    pub cyan: CustomColor,
    pub white: CustomColor,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TerminalColors {
    pub normal: AnsiColors,
    pub bright: AnsiColors,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Theme {
    pub accent: CustomColor,
    pub background: CustomColor,
    pub details: String,
    pub foreground: CustomColor,
    pub terminal_colors: TerminalColors,
}

#[derive(Debug)]
pub struct ThemeManager {
    pub themes: HashMap<String, Theme>,
}

impl ThemeManager {
    pub fn new() -> Self {
        let mut themes = HashMap::new();
        if let Ok(entries) = fs::read_dir("themes/") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext == "yaml" || ext == "yml" {
                        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                            if let Ok(content) = fs::read_to_string(&path) {
                                if let Ok(theme) = serde_yaml::from_str(&content) {
                                    themes.insert(name.to_string(), theme);
                                }
                            }
                        }
                    }
                }
            }
        }
        ThemeManager { themes }
    }
} 