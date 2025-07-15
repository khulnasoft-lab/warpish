//! Code Analysis
//! 
//! This module provides code analysis features, such as linting,
//! formatting, and symbol extraction.

use similar::TextDiff;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Failed to read file: {0}")]
    ReadFailed(std::io::Error),
}

#[derive(Debug, Clone)]
pub struct CodeBlock {
    pub language: String,
    pub content: String,
}

impl CodeBlock {
    pub fn new(language: &str, content: &str) -> Self {
        Self {
            language: language.to_string(),
            content: content.to_string(),
        }
    }

    pub fn diff(&self, other: &str) -> String {
        // FIX: Return a simple string representation of the diff
        let other_string = other.to_string();
        let diff = TextDiff::from_lines(&self.content, &other_string);
        // Convert diff to string representation
        let mut result = String::new();
        for change in diff.iter_all_changes() {
            match change.tag() {
                similar::ChangeTag::Delete => result.push_str(&format!("-{}", change)),
                similar::ChangeTag::Insert => result.push_str(&format!("+{}", change)),
                similar::ChangeTag::Equal => result.push_str(&format!(" {}", change)),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_code_block_from_file() {
        let mut file = tempfile::NamedTempFile::new().unwrap();
        writeln!(file, "fn main() {{}}").unwrap();

        let code_block = CodeBlock::from_file(file.path()).unwrap();
        assert_eq!(code_block.content.trim(), "fn main() {{}}");
    }

    #[test]
    fn test_code_block_diff() {
        let code_block = CodeBlock {
            content: "hello\nworld".to_string(),
            language: None,
        };

        let diff = code_block.diff("hello\nrust");
        assert!(diff.contains("-world"));
        assert!(diff.contains("+rust"));
    }
}
