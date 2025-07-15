//! Markdown Parser
//! 
//! This module converts tokens into an Abstract Syntax Tree (AST).

use super::{ast::*, lexer::Token, MarkdownError};

pub struct MarkdownParser {
    tokens: Vec<Token>,
    current: usize,
}

impl MarkdownParser {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }
    
    pub fn parse(&mut self, tokens: Vec<Token>) -> Result<Document, MarkdownError> {
        self.tokens = tokens;
        self.current = 0;
        
        let mut document = Document::new();
        
        while !self.is_at_end() {
            match self.parse_block()? {
                Some(block) => document.add_block(block),
                None => break,
            }
        }
        
        Ok(document)
    }
    
    fn parse_block(&mut self) -> Result<Option<Block>, MarkdownError> {
        match self.peek() {
            Token::Heading { level, content } => {
                self.advance();
                let inline_content = self.parse_inline_content(content)?;
                Ok(Some(Block::Heading(HeadingBlock::new(*level, inline_content))))
            }
            Token::CodeBlockStart { language } => {
                self.advance();
                let mut code_lines = Vec::new();
                let lang = language.clone();
                
                // Collect code content
                while !self.is_at_end() {
                    match self.peek() {
                        Token::CodeBlockContent(content) => {
                            code_lines.push(content.clone());
                            self.advance();
                        }
                        Token::CodeBlockEnd => {
                            self.advance();
                            break;
                        }
                        _ => break,
                    }
                }
                
                let code = code_lines.join("\n");
                let mut code_block = CodeBlock::new(code);
                
                if let Some(language) = lang {
                    code_block = code_block.with_language(language);
                }
                
                Ok(Some(Block::CodeBlock(code_block)))
            }
            Token::ListItemStart { marker, ordered } => {
                self.parse_list(*ordered)
            }
            Token::TableRow(cells) => {
                self.parse_table(cells.clone())
            }
            Token::QuoteStart => {
                self.parse_quote()
            }
            Token::ThematicBreak => {
                self.advance();
                Ok(Some(Block::ThematicBreak))
            }
            Token::Text(_) | Token::Bold(_) | Token::Italic(_) | Token::Code(_) | 
            Token::Link { .. } | Token::Image { .. } => {
                self.parse_paragraph()
            }
            Token::LineBreak => {
                self.advance();
                Ok(None) // Skip empty lines
            }
            Token::Eof => Ok(None),
            _ => {
                self.advance();
                Ok(None)
            }
        }
    }
    
    fn parse_paragraph(&mut self) -> Result<Option<Block>, MarkdownError> {
        let mut inline_elements = Vec::new();
        
        while !self.is_at_end() {
            match self.peek() {
                Token::Text(content) => {
                    inline_elements.push(Inline::Text(TextInline::new(content.clone())));
                    self.advance();
                }
                Token::Bold(content) => {
                    let inner_content = self.parse_inline_content(content)?;
                    inline_elements.push(Inline::Strong(StrongInline { content: inner_content }));
                    self.advance();
                }
                Token::Italic(content) => {
                    let inner_content = self.parse_inline_content(content)?;
                    inline_elements.push(Inline::Emphasis(EmphasisInline { content: inner_content }));
                    self.advance();
                }
                Token::Code(content) => {
                    inline_elements.push(Inline::Code(CodeInline { content: content.clone() }));
                    self.advance();
                }
                Token::Link { text, url, title } => {
                    let text_content = self.parse_inline_content(text)?;
                    let mut link = LinkInline::new(text_content, url.clone());
                    if let Some(title) = title {
                        link = link.with_title(title.clone());
                    }
                    inline_elements.push(Inline::Link(link));
                    self.advance();
                }
                Token::Image { alt, url, title } => {
                    let mut image = ImageInline::new(alt.clone(), url.clone());
                    if let Some(title) = title {
                        image = image.with_title(title.clone());
                    }
                    inline_elements.push(Inline::Image(image));
                    self.advance();
                }
                Token::LineBreak => {
                    // Check if this is the end of the paragraph
                    if self.check_ahead(1, &Token::LineBreak) || self.is_block_start_ahead() {
                        break;
                    }
                    inline_elements.push(Inline::SoftBreak);
                    self.advance();
                }
                _ => break,
            }
        }
        
        if inline_elements.is_empty() {
            Ok(None)
        } else {
            Ok(Some(Block::Paragraph(ParagraphBlock { content: inline_elements })))
        }
    }
    
    fn parse_list(&mut self, ordered: bool) -> Result<Option<Block>, MarkdownError> {
        let mut list = ListBlock::new(ordered);
        
        while !self.is_at_end() {
            match self.peek() {
                Token::ListItemStart { marker, .. } => {
                    self.advance();
                    let item_marker = marker.clone();
                    
                    // Parse the content of the list item
                    let mut item_content = Vec::new();
                    
                    // For now, treat list item content as a single paragraph
                    if let Some(Block::Paragraph(para)) = self.parse_paragraph()? {
                        item_content.push(Block::Paragraph(para));
                    }
                    
                    list.add_item(ListItem {
                        content: item_content,
                        marker: item_marker,
                    });
                }
                _ => break,
            }
        }
        
        Ok(Some(Block::List(list)))
    }
    
    fn parse_table(&mut self, first_row: Vec<String>) -> Result<Option<Block>, MarkdownError> {
        let mut table = TableBlock::new();
        
        // Parse header row
        for cell_content in first_row {
            let inline_content = self.parse_inline_content(&cell_content)?;
            table.add_header(TableCell { content: inline_content });
        }
        
        self.advance(); // consume the first row token
        
        // Parse remaining rows
        while !self.is_at_end() {
            match self.peek() {
                Token::TableRow(cells) => {
                    let mut row = Vec::new();
                    for cell_content in cells {
                        let inline_content = self.parse_inline_content(cell_content)?;
                        row.push(TableCell { content: inline_content });
                    }
                    table.add_row(row);
                    self.advance();
                }
                Token::TableSeparator => {
                    // Skip separator rows for now
                    self.advance();
                }
                _ => break,
            }
        }
        
        Ok(Some(Block::Table(table)))
    }
    
    fn parse_quote(&mut self) -> Result<Option<Block>, MarkdownError> {
        self.advance(); // consume QuoteStart
        
        let mut quote_content = Vec::new();
        
        // Parse content until we hit a non-quote block
        while !self.is_at_end() {
            match self.parse_block()? {
                Some(block) => quote_content.push(block),
                None => break,
            }
            
            // Check if next token is still part of quote
            if !matches!(self.peek(), Token::QuoteStart) {
                break;
            }
        }
        
        Ok(Some(Block::Quote(QuoteBlock { content: quote_content })))
    }
    
    fn parse_inline_content(&self, content: &str) -> Result<Vec<Inline>, MarkdownError> {
        // Simple implementation - just return as text
        // In a full implementation, this would parse inline markdown
        Ok(vec![Inline::Text(TextInline::new(content.to_string()))])
    }
    
    fn is_block_start_ahead(&self) -> bool {
        if self.current + 1 >= self.tokens.len() {
            return false;
        }
        
        matches!(self.tokens[self.current + 1], 
            Token::Heading { .. } | 
            Token::CodeBlockStart { .. } | 
            Token::ListItemStart { .. } | 
            Token::TableRow(_) | 
            Token::QuoteStart | 
            Token::ThematicBreak
        )
    }
    
    fn peek(&self) -> &Token {
        if self.is_at_end() {
            &Token::Eof
        } else {
            &self.tokens[self.current]
        }
    }
    
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    
    fn check_ahead(&self, offset: usize, token: &Token) -> bool {
        if self.current + offset >= self.tokens.len() {
            false
        } else {
            std::mem::discriminant(&self.tokens[self.current + offset]) == std::mem::discriminant(token)
        }
    }
    
    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.peek(), Token::Eof)
    }
}

impl Default for MarkdownParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::markdown_parser::lexer::MarkdownLexer;

    #[test]
    fn test_parse_heading() {
        let mut lexer = MarkdownLexer::new();
        let tokens = lexer.tokenize("# Hello World").unwrap();
        
        let mut parser = MarkdownParser::new();
        let document = parser.parse(tokens).unwrap();
        
        assert_eq!(document.blocks.len(), 1);
        assert!(matches!(document.blocks[0], Block::Heading(_)));
    }
    
    #[test]
    fn test_parse_paragraph() {
        let mut lexer = MarkdownLexer::new();
        let tokens = lexer.tokenize("This is a paragraph with **bold** text.").unwrap();
        
        let mut parser = MarkdownParser::new();
        let document = parser.parse(tokens).unwrap();
        
        assert_eq!(document.blocks.len(), 1);
        assert!(matches!(document.blocks[0], Block::Paragraph(_)));
    }
    
    #[test]
    fn test_parse_code_block() {
        let mut lexer = MarkdownLexer::new();
        let tokens = lexer.tokenize("```rust
fn main() {}
```").unwrap();
        
        let mut parser = MarkdownParser::new();
        let document = parser.parse(tokens).unwrap();
        
        assert_eq!(document.blocks.len(), 1);
        assert!(matches!(document.blocks[0], Block::CodeBlock(_)));
    }
}
