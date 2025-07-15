//! Abstract Syntax Tree definitions for Markdown parsing
//! 
//! This module defines the AST nodes used to represent parsed markdown content.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Document {
    pub blocks: Vec<Block>,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Block {
    Heading(HeadingBlock),
    Paragraph(ParagraphBlock),
    CodeBlock(CodeBlock),
    List(ListBlock),
    Table(TableBlock),
    Quote(QuoteBlock),
    ThematicBreak,
    Html(HtmlBlock),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HeadingBlock {
    pub level: u8, // 1-6
    pub content: Vec<Inline>,
    pub id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParagraphBlock {
    pub content: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub code: String,
    pub line_numbers: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListBlock {
    pub items: Vec<ListItem>,
    pub ordered: bool,
    pub start: Option<u32>,
    pub tight: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListItem {
    pub content: Vec<Block>,
    pub marker: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableBlock {
    pub headers: Vec<TableCell>,
    pub rows: Vec<Vec<TableCell>>,
    pub alignments: Vec<TableAlignment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TableCell {
    pub content: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TableAlignment {
    Left,
    Right,
    Center,
    None,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuoteBlock {
    pub content: Vec<Block>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HtmlBlock {
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Inline {
    Text(TextInline),
    Emphasis(EmphasisInline),
    Strong(StrongInline),
    Code(CodeInline),
    Link(LinkInline),
    Image(ImageInline),
    LineBreak,
    SoftBreak,
    Html(HtmlInline),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextInline {
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EmphasisInline {
    pub content: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StrongInline {
    pub content: Vec<Inline>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CodeInline {
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkInline {
    pub content: Vec<Inline>,
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ImageInline {
    pub alt: String,
    pub url: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HtmlInline {
    pub content: String,
}

impl Document {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_block(&mut self, block: Block) {
        self.blocks.push(block);
    }
    
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }
}

impl Block {
    pub fn is_heading(&self) -> bool {
        matches!(self, Block::Heading(_))
    }
    
    pub fn is_code_block(&self) -> bool {
        matches!(self, Block::CodeBlock(_))
    }
    
    pub fn is_table(&self) -> bool {
        matches!(self, Block::Table(_))
    }
}

impl HeadingBlock {
    pub fn new(level: u8, content: Vec<Inline>) -> Self {
        Self {
            level: level.min(6).max(1),
            content,
            id: None,
        }
    }
    
    pub fn with_id(mut self, id: String) -> Self {
        self.id = Some(id);
        self
    }
}

impl CodeBlock {
    pub fn new(code: String) -> Self {
        Self {
            language: None,
            code,
            line_numbers: false,
        }
    }
    
    pub fn with_language(mut self, language: String) -> Self {
        self.language = Some(language);
        self
    }
    
    pub fn with_line_numbers(mut self, show: bool) -> Self {
        self.line_numbers = show;
        self
    }
}

impl ListBlock {
    pub fn new(ordered: bool) -> Self {
        Self {
            items: Vec::new(),
            ordered,
            start: None,
            tight: true,
        }
    }
    
    pub fn add_item(&mut self, item: ListItem) {
        self.items.push(item);
    }
}

impl TableBlock {
    pub fn new() -> Self {
        Self {
            headers: Vec::new(),
            rows: Vec::new(),
            alignments: Vec::new(),
        }
    }
    
    pub fn add_header(&mut self, cell: TableCell) {
        self.headers.push(cell);
    }
    
    pub fn add_row(&mut self, row: Vec<TableCell>) {
        self.rows.push(row);
    }
    
    pub fn set_alignments(&mut self, alignments: Vec<TableAlignment>) {
        self.alignments = alignments;
    }
}

impl TextInline {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}

impl LinkInline {
    pub fn new(content: Vec<Inline>, url: String) -> Self {
        Self {
            content,
            url,
            title: None,
        }
    }
    
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
}

impl ImageInline {
    pub fn new(alt: String, url: String) -> Self {
        Self {
            alt,
            url,
            title: None,
        }
    }
    
    pub fn with_title(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }
}
