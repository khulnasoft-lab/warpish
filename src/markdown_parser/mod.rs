//! Markdown Parser
//! 
//! This module provides comprehensive markdown parsing and rendering capabilities
//! for the terminal, including syntax highlighting, table rendering, and interactive elements.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

pub mod ast;
pub mod lexer;
pub mod parser;
pub mod renderer;
pub mod themes;

pub use ast::*;
pub use lexer::*;
pub use parser::*;
pub use renderer::*;
pub use themes::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkdownError {
    ParseError(String),
    RenderError(String),
    InvalidSyntax(String),
}

impl fmt::Display for MarkdownError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MarkdownError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            MarkdownError::RenderError(msg) => write!(f, "Render error: {}", msg),
            MarkdownError::InvalidSyntax(msg) => write!(f, "Invalid syntax: {}", msg),
        }
    }
}

impl std::error::Error for MarkdownError {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarkdownConfig {
    pub syntax_highlighting: bool,
    pub table_rendering: bool,
    pub link_highlighting: bool,
    pub code_block_theme: String,
    pub max_width: Option<usize>,
    pub indent_size: usize,
}

impl Default for MarkdownConfig {
    fn default() -> Self {
        Self {
            syntax_highlighting: true,
            table_rendering: true,
            link_highlighting: true,
            code_block_theme: "monokai".to_string(),
            max_width: Some(80),
            indent_size: 2,
        }
    }
}

pub struct MarkdownProcessor {
    config: MarkdownConfig,
    lexer: MarkdownLexer,
    parser: MarkdownParser,
    renderer: TerminalRenderer,
}

impl MarkdownProcessor {
    pub fn new() -> Self {
        let config = MarkdownConfig::default();
        Self {
            lexer: MarkdownLexer::new(),
            parser: MarkdownParser::new(),
            renderer: TerminalRenderer::new(&config),
            config,
        }
    }
    
    pub fn with_config(config: MarkdownConfig) -> Self {
        Self {
            lexer: MarkdownLexer::new(),
            parser: MarkdownParser::new(),
            renderer: TerminalRenderer::new(&config),
            config,
        }
    }
    
    pub fn parse(&mut self, input: &str) -> Result<Document, MarkdownError> {
        let tokens = self.lexer.tokenize(input)?;
        self.parser.parse(tokens)
    }
    
    pub fn render(&self, document: &Document) -> Result<String, MarkdownError> {
        self.renderer.render(document)
    }
    
    pub fn process(&mut self, input: &str) -> Result<String, MarkdownError> {
        let document = self.parse(input)?;
        self.render(&document)
    }
}

impl Default for MarkdownProcessor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_markdown_processing() {
        let mut processor = MarkdownProcessor::new();
        let input = "# Hello World\n\nThis is a **bold** text.";
        let result = processor.process(input);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_code_block_processing() {
        let mut processor = MarkdownProcessor::new();
        let input = "```rust\nfn main() {\n    println!(\"Hello, World!\");\n}\n```";
        let result = processor.process(input);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_table_processing() {
        let mut processor = MarkdownProcessor::new();
        let input = "| Name | Age |\n|------|-----|\n| John | 25  |\n| Jane | 30  |";
        let result = processor.process(input);
        assert!(result.is_ok());
    }
}
