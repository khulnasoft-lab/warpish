//! Code Analysis
//! 
//! This module provides code analysis features, such as linting,
//! formatting, and symbol extraction.

use similar::{ChangeTag, TextDiff};
use std::fs;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Failed to read file: {0}")]
    ReadFailed(std::io::Error),
}

/// Represents a piece of code to be analyzed.
pub struct CodeBlock {
    pub content: String,
    pub language: Option<String>,
}

impl CodeBlock {
    pub fn from_file(path: &Path) -> Result<Self, CodeError> {
        if !path.exists() {
            return Err(CodeError::NotFound(path.to_string_lossy().to_string()));
        }

        let content = fs::read_to_string(path).map_err(CodeError::ReadFailed)?;
        let language = path.extension().and_then(|s| s.to_str()).map(|s| s.to_string());

        Ok(Self { content, language })
    }

    /// Diffs the code block against another string.
    pub fn diff(&self, other: &str) -> String {
        let diff = TextDiff::from_lines(&self.content, other);
        let mut result = String::new();

        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            result.push_str(&format!("{}{}", sign, change));
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
