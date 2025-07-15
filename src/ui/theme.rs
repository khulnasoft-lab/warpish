use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};
use crate::config::theme::{Theme as ConfigTheme, load_theme};
use log::{warn, info};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomColor(pub u8, pub u8, pub u8);

impl CustomColor {
    /// Convert to hex string representation
    pub fn to_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.0, self.1, self.2)
    }
    
    /// Create from RGB values
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        CustomColor(r, g, b)
    }
    
    /// Create from hex string
    pub fn from_hex(hex: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Invalid hex color format".into());
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        
        Ok(CustomColor(r, g, b))
    }
}

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
        let mut manager = ThemeManager {
            themes: HashMap::new(),
        };
        manager.load_themes();
        manager
    }
    
    /// Load themes from the themes directory
    pub fn load_themes(&mut self) {
        self.themes.clear();
        
        if let Ok(entries) = fs::read_dir("themes/") {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if ext == "yaml" || ext == "yml" {
                        if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                            match load_theme(&path) {
                                Ok(config_theme) => {
                                    // Convert ConfigTheme to our Theme format
                                    if let Ok(theme) = self.convert_config_theme(&config_theme) {
                                        self.themes.insert(name.to_string(), theme);
                                        info!("Loaded theme: {}", name);
                                    } else {
                                        warn!("Failed to convert theme: {}", name);
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to load theme {}: {}", name, e);
                                }
                            }
                        }
                    }
                }
            }
        } else {
            warn!("Could not read themes directory");
        }
        
        info!("Loaded {} themes", self.themes.len());
    }
    
    /// Get a theme by name
    pub fn get_theme(&self, name: &str) -> Option<&Theme> {
        self.themes.get(name)
    }
    
    /// Get all available theme names
    pub fn get_theme_names(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }
    
    /// Get default theme (fallback)
    pub fn get_default_theme(&self) -> Theme {
        // Return a default theme if no themes are loaded
        Theme {
            accent: CustomColor(70, 130, 180),
            background: CustomColor(30, 30, 30),
            details: "Default theme".to_string(),
            foreground: CustomColor(255, 255, 255),
            terminal_colors: TerminalColors {
                normal: AnsiColors {
                    black: CustomColor(0, 0, 0),
                    red: CustomColor(255, 0, 0),
                    green: CustomColor(0, 255, 0),
                    yellow: CustomColor(255, 255, 0),
                    blue: CustomColor(0, 0, 255),
                    magenta: CustomColor(255, 0, 255),
                    cyan: CustomColor(0, 255, 255),
                    white: CustomColor(255, 255, 255),
                },
                bright: AnsiColors {
                    black: CustomColor(85, 85, 85),
                    red: CustomColor(255, 85, 85),
                    green: CustomColor(85, 255, 85),
                    yellow: CustomColor(255, 255, 85),
                    blue: CustomColor(85, 85, 255),
                    magenta: CustomColor(255, 85, 255),
                    cyan: CustomColor(85, 255, 255),
                    white: CustomColor(255, 255, 255),
                },
            },
        }
    }
    
    /// Convert ConfigTheme to our Theme format
    fn convert_config_theme(&self, config_theme: &ConfigTheme) -> Result<Theme, Box<dyn std::error::Error>> {
        Ok(Theme {
            accent: self.parse_color(&config_theme.colors.selection.background)?,
            background: self.parse_color(&config_theme.colors.primary.background)?,
            details: "Converted theme".to_string(),
            foreground: self.parse_color(&config_theme.colors.primary.foreground)?,
            terminal_colors: TerminalColors {
                normal: AnsiColors {
                    black: self.parse_color(&config_theme.colors.normal.black)?,
                    red: self.parse_color(&config_theme.colors.normal.red)?,
                    green: self.parse_color(&config_theme.colors.normal.green)?,
                    yellow: self.parse_color(&config_theme.colors.normal.yellow)?,
                    blue: self.parse_color(&config_theme.colors.normal.blue)?,
                    magenta: self.parse_color(&config_theme.colors.normal.magenta)?,
                    cyan: self.parse_color(&config_theme.colors.normal.cyan)?,
                    white: self.parse_color(&config_theme.colors.normal.white)?,
                },
                bright: AnsiColors {
                    black: self.parse_color(&config_theme.colors.bright.black)?,
                    red: self.parse_color(&config_theme.colors.bright.red)?,
                    green: self.parse_color(&config_theme.colors.bright.green)?,
                    yellow: self.parse_color(&config_theme.colors.bright.yellow)?,
                    blue: self.parse_color(&config_theme.colors.bright.blue)?,
                    magenta: self.parse_color(&config_theme.colors.bright.magenta)?,
                    cyan: self.parse_color(&config_theme.colors.bright.cyan)?,
                    white: self.parse_color(&config_theme.colors.bright.white)?,
                },
            },
        })
    }
    
    /// Parse hex color string to CustomColor
    fn parse_color(&self, hex: &str) -> Result<CustomColor, Box<dyn std::error::Error>> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Err("Invalid hex color format".into());
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16)?;
        let g = u8::from_str_radix(&hex[2..4], 16)?;
        let b = u8::from_str_radix(&hex[4..6], 16)?;
        
        Ok(CustomColor(r, g, b))
    }
}
