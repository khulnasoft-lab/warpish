use std::collections::HashSet;
use std::env;

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Command,
    Subcommand, // For future use
    Flag,
    Argument,
    Error,
}

#[derive(Debug)]
pub struct Token {
    pub content: String,
    pub token_type: TokenType,
    pub start: usize,
    pub end: usize,
}

pub struct SyntaxParser {
    known_commands: HashSet<String>,
}

impl SyntaxParser {
    /// Creates a new parser, caching all executables found in the system's PATH.
    pub fn new() -> Self {
        let mut known_commands = HashSet::new();
        if let Ok(paths) = env::var("PATH") {
            for path in env::split_paths(&paths) {
                if let Ok(iter) = std::fs::read_dir(path) {
                    for entry in iter.flatten() {
                        if let Some(name) = entry.file_name().to_str() {
                            // A simple check if it's executable might be needed on Unix
                            known_commands.insert(name.to_string());
                        }
                    }
                }
            }
        }
        log::info!("Cached {} commands from PATH for syntax highlighting.", known_commands.len());
        Self { known_commands }
    }

    /// Parses a line of text into a vector of styled tokens.
    pub fn parse(&self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current_pos = 0;

        for word in text.split_whitespace() {
            let start = text[current_pos..].find(word).unwrap_or(0) + current_pos;
            let end = start + word.len();
            current_pos = end;

            let token_type = if tokens.is_empty() {
                // First token is the main command
                if self.known_commands.contains(word) {
                    TokenType::Command
                } else {
                    TokenType::Error
                }
            } else if word.starts_with('-') {
                TokenType::Flag
            } else {
                TokenType::Argument
            };
            
            tokens.push(Token {
                content: word.to_string(),
                token_type,
                start,
                end,
            });
        }
        tokens
    }
} 