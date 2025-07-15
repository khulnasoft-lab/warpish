//! Markdown Themes
//! 
//! This module provides theming support for markdown rendering.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownTheme {
    pub name: String,
    pub heading_colors: [String; 6],
    pub text_color: String,
    pub emphasis_color: String,
    pub strong_color: String,
    pub code_color: String,
    pub code_background: String,
    pub link_color: String,
    pub quote_color: String,
    pub table_border_color: String,
    pub list_marker_color: String,
}

impl Default for MarkdownTheme {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
            heading_colors: [
                "\\x1b[94m".to_string(),  // bright blue
                "\\x1b[92m".to_string(),  // bright green
                "\\x1b[93m".to_string(),  // bright yellow
                "\\x1b[95m".to_string(),  // bright magenta
                "\\x1b[96m".to_string(),  // bright cyan
                "\\x1b[97m".to_string(),  // bright white
            ],
            text_color: "\\x1b[97m".to_string(),        // bright white
            emphasis_color: "\\x1b[3m".to_string(),     // italic
            strong_color: "\\x1b[1m".to_string(),       // bold
            code_color: "\\x1b[92m".to_string(),        // bright green
            code_background: "\\x1b[40m".to_string(),   // black background
            link_color: "\\x1b[94m".to_string(),        // bright blue
            quote_color: "\\x1b[90m".to_string(),       // bright black
            table_border_color: "\\x1b[97m".to_string(), // bright white
            list_marker_color: "\\x1b[96m".to_string(),  // bright cyan
        }
    }
}

impl MarkdownTheme {
    pub fn monokai() -> Self {
        Self {
            name: "monokai".to_string(),
            heading_colors: [
                "\\x1b[38;5;81m".to_string(),   // cyan
                "\\x1b[38;5;118m".to_string(),  // green
                "\\x1b[38;5;227m".to_string(),  // yellow
                "\\x1b[38;5;141m".to_string(),  // purple
                "\\x1b[38;5;208m".to_string(),  // orange
                "\\x1b[38;5;15m".to_string(),   // white
            ],
            text_color: "\\x1b[38;5;15m".to_string(),     // white
            emphasis_color: "\\x1b[3;38;5;15m".to_string(), // italic white
            strong_color: "\\x1b[1;38;5;15m".to_string(),   // bold white
            code_color: "\\x1b[38;5;118m".to_string(),      // green
            code_background: "\\x1b[48;5;235m".to_string(), // dark grey
            link_color: "\\x1b[38;5;81m".to_string(),       // cyan
            quote_color: "\\x1b[38;5;102m".to_string(),     // grey
            table_border_color: "\\x1b[38;5;15m".to_string(), // white
            list_marker_color: "\\x1b[38;5;208m".to_string(),  // orange
        }
    }
    
    pub fn solarized_dark() -> Self {
        Self {
            name: "solarized_dark".to_string(),
            heading_colors: [
                "\\x1b[38;5;33m".to_string(),   // blue
                "\\x1b[38;5;64m".to_string(),   // green
                "\\x1b[38;5;136m".to_string(),  // yellow
                "\\x1b[38;5;125m".to_string(),  // magenta
                "\\x1b[38;5;37m".to_string(),   // cyan
                "\\x1b[38;5;230m".to_string(),  // base3
            ],
            text_color: "\\x1b[38;5;230m".to_string(),      // base3
            emphasis_color: "\\x1b[3;38;5;230m".to_string(), // italic base3
            strong_color: "\\x1b[1;38;5;230m".to_string(),   // bold base3
            code_color: "\\x1b[38;5;64m".to_string(),       // green
            code_background: "\\x1b[48;5;235m".to_string(), // base02
            link_color: "\\x1b[38;5;33m".to_string(),       // blue
            quote_color: "\\x1b[38;5;244m".to_string(),     // base0
            table_border_color: "\\x1b[38;5;244m".to_string(), // base0
            list_marker_color: "\\x1b[38;5;166m".to_string(),  // orange
        }
    }
    
    pub fn solarized_light() -> Self {
        Self {
            name: "solarized_light".to_string(),
            heading_colors: [
                "\\x1b[38;5;33m".to_string(),   // blue
                "\\x1b[38;5;64m".to_string(),   // green
                "\\x1b[38;5;136m".to_string(),  // yellow
                "\\x1b[38;5;125m".to_string(),  // magenta
                "\\x1b[38;5;37m".to_string(),   // cyan
                "\\x1b[38;5;235m".to_string(),  // base02
            ],
            text_color: "\\x1b[38;5;235m".to_string(),      // base02
            emphasis_color: "\\x1b[3;38;5;235m".to_string(), // italic base02
            strong_color: "\\x1b[1;38;5;235m".to_string(),   // bold base02
            code_color: "\\x1b[38;5;64m".to_string(),       // green
            code_background: "\\x1b[48;5;230m".to_string(), // base3
            link_color: "\\x1b[38;5;33m".to_string(),       // blue
            quote_color: "\\x1b[38;5;244m".to_string(),     // base0
            table_border_color: "\\x1b[38;5;244m".to_string(), // base0
            list_marker_color: "\\x1b[38;5;166m".to_string(),  // orange
        }
    }
    
    pub fn github() -> Self {
        Self {
            name: "github".to_string(),
            heading_colors: [
                "\\x1b[38;5;4m".to_string(),    // blue
                "\\x1b[38;5;2m".to_string(),    // green
                "\\x1b[38;5;3m".to_string(),    // yellow
                "\\x1b[38;5;5m".to_string(),    // magenta
                "\\x1b[38;5;6m".to_string(),    // cyan
                "\\x1b[38;5;8m".to_string(),    // grey
            ],
            text_color: "\\x1b[38;5;0m".to_string(),        // black
            emphasis_color: "\\x1b[3;38;5;0m".to_string(),   // italic black
            strong_color: "\\x1b[1;38;5;0m".to_string(),     // bold black
            code_color: "\\x1b[38;5;1m".to_string(),        // red
            code_background: "\\x1b[48;5;7m".to_string(),   // light grey
            link_color: "\\x1b[38;5;4m".to_string(),        // blue
            quote_color: "\\x1b[38;5;8m".to_string(),       // grey
            table_border_color: "\\x1b[38;5;8m".to_string(), // grey
            list_marker_color: "\\x1b[38;5;0m".to_string(),  // black
        }
    }
    
    pub fn dracula() -> Self {
        Self {
            name: "dracula".to_string(),
            heading_colors: [
                "\\x1b[38;5;141m".to_string(),  // purple
                "\\x1b[38;5;84m".to_string(),   // green
                "\\x1b[38;5;228m".to_string(),  // yellow
                "\\x1b[38;5;212m".to_string(),  // pink
                "\\x1b[38;5;117m".to_string(),  // cyan
                "\\x1b[38;5;15m".to_string(),   // white
            ],
            text_color: "\\x1b[38;5;15m".to_string(),       // white
            emphasis_color: "\\x1b[3;38;5;15m".to_string(),  // italic white
            strong_color: "\\x1b[1;38;5;15m".to_string(),    // bold white
            code_color: "\\x1b[38;5;84m".to_string(),       // green
            code_background: "\\x1b[48;5;235m".to_string(), // dark grey
            link_color: "\\x1b[38;5;117m".to_string(),      // cyan
            quote_color: "\\x1b[38;5;102m".to_string(),     // grey
            table_border_color: "\\x1b[38;5;15m".to_string(), // white
            list_marker_color: "\\x1b[38;5;212m".to_string(),  // pink
        }
    }
    
    pub fn by_name(name: &str) -> Self {
        match name {
            "monokai" => Self::monokai(),
            "solarized_dark" => Self::solarized_dark(),
            "solarized_light" => Self::solarized_light(),
            "github" => Self::github(),
            "dracula" => Self::dracula(),
            _ => Self::default(),
        }
    }
    
    pub fn available_themes() -> Vec<String> {
        vec![
            "default".to_string(),
            "monokai".to_string(),
            "solarized_dark".to_string(),
            "solarized_light".to_string(),
            "github".to_string(),
            "dracula".to_string(),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_theme() {
        let theme = MarkdownTheme::default();
        assert_eq!(theme.name, "default");
        assert_eq!(theme.heading_colors.len(), 6);
    }
    
    #[test]
    fn test_theme_by_name() {
        let monokai = MarkdownTheme::by_name("monokai");
        assert_eq!(monokai.name, "monokai");
        
        let unknown = MarkdownTheme::by_name("unknown");
        assert_eq!(unknown.name, "default");
    }
    
    #[test]
    fn test_available_themes() {
        let themes = MarkdownTheme::available_themes();
        assert!(themes.contains(&"default".to_string()));
        assert!(themes.contains(&"monokai".to_string()));
        assert!(themes.contains(&"github".to_string()));
    }
}
