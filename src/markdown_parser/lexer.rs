//! Markdown Lexer
//! 
//! This module tokenizes markdown text into a stream of tokens for parsing.

use super::MarkdownError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Token {
    // Block-level tokens
    Heading { level: u8, content: String },
    CodeBlockStart { language: Option<String> },
    CodeBlockEnd,
    CodeBlockContent(String),
    ListItemStart { marker: String, ordered: bool },
    TableSeparator,
    TableRow(Vec<String>),
    QuoteStart,
    ThematicBreak,
    
    // Inline tokens
    Text(String),
    Bold(String),
    Italic(String),
    Code(String),
    Link { text: String, url: String, title: Option<String> },
    Image { alt: String, url: String, title: Option<String> },
    
    // Structural tokens
    LineBreak,
    SoftBreak,
    Paragraph,
    Eof,
}

#[derive(Debug, Clone)]
pub struct TokenPosition {
    pub line: usize,
    pub column: usize,
    pub offset: usize,
}

#[derive(Debug, Clone)]
pub struct TokenWithPosition {
    pub token: Token,
    pub position: TokenPosition,
}

pub struct MarkdownLexer {
    input: String,
    position: usize,
    line: usize,
    column: usize,
    tokens: Vec<TokenWithPosition>,
}

impl MarkdownLexer {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            position: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }
    
    pub fn tokenize(&mut self, input: &str) -> Result<Vec<Token>, MarkdownError> {
        self.input = input.to_string();
        self.position = 0;
        self.line = 1;
        self.column = 1;
        self.tokens.clear();
        
        while !self.is_at_end() {
            self.scan_token()?;
        }
        
        self.add_token(Token::Eof);
        Ok(self.tokens.iter().map(|t| t.token.clone()).collect())
    }
    
    fn scan_token(&mut self) -> Result<(), MarkdownError> {
        let start_pos = self.position;
        
        // Skip whitespace at line start to check for block elements
        if self.column == 1 {
            self.skip_whitespace();
        }
        
        let ch = self.advance();
        
        match ch {
            '\n' => {
                self.add_token(Token::LineBreak);
                self.line += 1;
                self.column = 1;
            }
            '#' if self.column == 1 || self.previous_char() == '\n' => {
                self.scan_heading()?;
            }
            '`' => {
                if self.check('`') && self.check_ahead(1, '`') {
                    self.scan_code_block()?;
                } else {
                    self.scan_inline_code()?;
                }
            }
            '*' => {
                if self.check('*') {
                    self.scan_bold()?;
                } else {
                    self.scan_italic()?;
                }
            }
            '_' => {
                if self.check('_') {
                    self.scan_bold()?;
                } else {
                    self.scan_italic()?;
                }
            }
            '[' => {
                self.scan_link_or_image()?;
            }
            '!' if self.check('[') => {
                self.advance(); // consume '['
                self.scan_image()?;
            }
            '-' if self.is_list_marker() => {
                self.scan_list_item()?;
            }
            '|' => {
                self.scan_table_row()?;
            }
            '>' if self.column == 1 => {
                self.scan_quote()?;
            }
            _ => {
                // Go back and scan as text
                self.position = start_pos;
                self.scan_text()?;
            }
        }
        
        Ok(())
    }
    
    fn scan_heading(&mut self) -> Result<(), MarkdownError> {
        let mut level = 1;
        
        while self.check('#') && level < 6 {
            self.advance();
            level += 1;
        }
        
        if !self.check(' ') && !self.is_at_end() {
            // Not a valid heading, treat as text
            return self.scan_text();
        }
        
        self.advance(); // consume space
        let content = self.scan_until_newline();
        
        self.add_token(Token::Heading { level, content });
        Ok(())
    }
    
    fn scan_code_block(&mut self) -> Result<(), MarkdownError> {
        // Consume the opening ```
        self.advance(); // second `
        self.advance(); // third `
        
        // Get language if present
        let language = if !self.check('\n') && !self.is_at_end() {
            Some(self.scan_until_newline())
        } else {
            None
        };
        
        self.add_token(Token::CodeBlockStart { language });
        
        // Scan code content
        while !self.is_at_end() {
            if self.check('`') && self.check_ahead(1, '`') && self.check_ahead(2, '`') {
                // Found closing ```
                self.advance(); // first `
                self.advance(); // second `
                self.advance(); // third `
                break;
            }
            
            let content = self.scan_until_newline();
            self.add_token(Token::CodeBlockContent(content));
            
            if self.check('\n') {
                self.advance();
                self.line += 1;
                self.column = 1;
            }
        }
        
        self.add_token(Token::CodeBlockEnd);
        Ok(())
    }
    
    fn scan_inline_code(&mut self) -> Result<(), MarkdownError> {
        let content = self.scan_until_char('`');
        if !self.is_at_end() {
            self.advance(); // consume closing `
        }
        
        self.add_token(Token::Code(content));
        Ok(())
    }
    
    fn scan_bold(&mut self) -> Result<(), MarkdownError> {
        let delimiter = self.previous_char();
        self.advance(); // consume second delimiter
        
        let content = self.scan_until_double_char(delimiter);
        
        if !self.is_at_end() {
            self.advance(); // consume first closing delimiter
            self.advance(); // consume second closing delimiter
        }
        
        self.add_token(Token::Bold(content));
        Ok(())
    }
    
    fn scan_italic(&mut self) -> Result<(), MarkdownError> {
        let delimiter = self.previous_char();
        let content = self.scan_until_char(delimiter);
        
        if !self.is_at_end() {
            self.advance(); // consume closing delimiter
        }
        
        self.add_token(Token::Italic(content));
        Ok(())
    }
    
    fn scan_link_or_image(&mut self) -> Result<(), MarkdownError> {
        let text = self.scan_until_char(']');
        
        if !self.is_at_end() {
            self.advance(); // consume ']'
        }
        
        if self.check('(') {
            self.advance(); // consume '('
            let url = self.scan_until_char(')');
            
            if !self.is_at_end() {
                self.advance(); // consume ')'
            }
            
            self.add_token(Token::Link { text, url, title: None });
        } else {
            // Treat as regular text
            self.add_token(Token::Text(format!("[{}]", text)));
        }
        
        Ok(())
    }
    
    fn scan_image(&mut self) -> Result<(), MarkdownError> {
        let alt = self.scan_until_char(']');
        
        if !self.is_at_end() {
            self.advance(); // consume ']'
        }
        
        if self.check('(') {
            self.advance(); // consume '('
            let url = self.scan_until_char(')');
            
            if !self.is_at_end() {
                self.advance(); // consume ')'
            }
            
            self.add_token(Token::Image { alt, url, title: None });
        } else {
            // Treat as regular text
            self.add_token(Token::Text(format!("![{}]", alt)));
        }
        
        Ok(())
    }
    
    fn scan_list_item(&mut self) -> Result<(), MarkdownError> {
        let marker = self.previous_char().to_string();
        
        if self.check(' ') {
            self.advance(); // consume space
        }
        
        self.add_token(Token::ListItemStart { marker, ordered: false });
        Ok(())
    }
    
    fn scan_table_row(&mut self) -> Result<(), MarkdownError> {
        let mut cells = Vec::new();
        let mut current_cell = String::new();
        
        while !self.is_at_end() && !self.check('\n') {
            if self.check('|') {
                cells.push(current_cell.trim().to_string());
                current_cell.clear();
                self.advance();
            } else {
                current_cell.push(self.advance());
            }
        }
        
        if !current_cell.is_empty() {
            cells.push(current_cell.trim().to_string());
        }
        
        self.add_token(Token::TableRow(cells));
        Ok(())
    }
    
    fn scan_quote(&mut self) -> Result<(), MarkdownError> {
        if self.check(' ') {
            self.advance(); // consume space
        }
        
        self.add_token(Token::QuoteStart);
        Ok(())
    }
    
    fn scan_text(&mut self) -> Result<(), MarkdownError> {
        let mut text = String::new();
        
        while !self.is_at_end() && !self.is_special_char() {
            text.push(self.advance());
        }
        
        if !text.is_empty() {
            self.add_token(Token::Text(text));
        }
        
        Ok(())
    }
    
    fn scan_until_newline(&mut self) -> String {
        let mut content = String::new();
        
        while !self.is_at_end() && !self.check('\n') {
            content.push(self.advance());
        }
        
        content
    }
    
    fn scan_until_char(&mut self, ch: char) -> String {
        let mut content = String::new();
        
        while !self.is_at_end() && !self.check(ch) {
            content.push(self.advance());
        }
        
        content
    }
    
    fn scan_until_double_char(&mut self, ch: char) -> String {
        let mut content = String::new();
        
        while !self.is_at_end() && !(self.check(ch) && self.check_ahead(1, ch)) {
            content.push(self.advance());
        }
        
        content
    }
    
    fn is_special_char(&self) -> bool {
        matches!(self.peek(), '*' | '_' | '`' | '[' | ']' | '!' | '(' | ')' | '\n' | '|' | '#' | '>' | '-')
    }
    
    fn is_list_marker(&self) -> bool {
        (self.column == 1 || self.previous_char() == '\n') && 
        (self.check(' ') || self.check_ahead(1, ' '))
    }
    
    fn skip_whitespace(&mut self) {
        while self.check(' ') || self.check('\t') {
            self.advance();
        }
    }
    
    fn advance(&mut self) -> char {
        let ch = self.peek();
        self.position += 1;
        self.column += 1;
        ch
    }
    
    fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.input.chars().nth(self.position).unwrap_or('\0')
        }
    }
    
    fn previous_char(&self) -> char {
        if self.position == 0 {
            '\0'
        } else {
            self.input.chars().nth(self.position - 1).unwrap_or('\0')
        }
    }
    
    fn check(&self, ch: char) -> bool {
        self.peek() == ch
    }
    
    fn check_ahead(&self, offset: usize, ch: char) -> bool {
        if self.position + offset >= self.input.len() {
            false
        } else {
            self.input.chars().nth(self.position + offset).unwrap_or('\0') == ch
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
    
    fn add_token(&mut self, token: Token) {
        let position = TokenPosition {
            line: self.line,
            column: self.column,
            offset: self.position,
        };
        
        self.tokens.push(TokenWithPosition { token, position });
    }
}

impl Default for MarkdownLexer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heading_tokenization() {
        let mut lexer = MarkdownLexer::new();
        let tokens = lexer.tokenize("# Hello World").unwrap();
        
        assert_eq!(tokens.len(), 2); // Heading + EOF
        match &tokens[0] {
            Token::Heading { level, content } => {
                assert_eq!(*level, 1);
                assert_eq!(content, "Hello World");
            }
            _ => panic!("Expected heading token"),
        }
    }
    
    #[test]
    fn test_code_block_tokenization() {
        let mut lexer = MarkdownLexer::new();
        let tokens = lexer.tokenize("```rust\nfn main() {}\n```").unwrap();
        
        assert!(matches!(tokens[0], Token::CodeBlockStart { .. }));
        assert!(matches!(tokens[1], Token::CodeBlockContent(_)));
        assert!(matches!(tokens[2], Token::CodeBlockEnd));
    }
    
    #[test]
    fn test_inline_formatting() {
        let mut lexer = MarkdownLexer::new();
        let tokens = lexer.tokenize("**bold** and *italic*").unwrap();
        
        assert!(tokens.iter().any(|t| matches!(t, Token::Bold(_))));
        assert!(tokens.iter().any(|t| matches!(t, Token::Italic(_))));
    }
}
