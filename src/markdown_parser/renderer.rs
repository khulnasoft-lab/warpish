//! Terminal Renderer for Markdown
//! 
//! This module renders markdown AST to terminal-formatted text with colors and styling.

use super::{ast::*, themes::MarkdownTheme, MarkdownConfig, MarkdownError};
use std::fmt::Write;

/// ANSI color codes for terminal styling
mod ansi {
    pub const RESET: &str = "\x1b[0m";
    pub const BOLD: &str = "\x1b[1m";
    pub const ITALIC: &str = "\x1b[3m";
    pub const UNDERLINE: &str = "\x1b[4m";
    
    // Colors
    pub const BLACK: &str = "\x1b[30m";
    pub const RED: &str = "\x1b[31m";
    pub const GREEN: &str = "\x1b[32m";
    pub const YELLOW: &str = "\x1b[33m";
    pub const BLUE: &str = "\x1b[34m";
    pub const MAGENTA: &str = "\x1b[35m";
    pub const CYAN: &str = "\x1b[36m";
    pub const WHITE: &str = "\x1b[37m";
    
    // Bright colors
    pub const BRIGHT_BLACK: &str = "\x1b[90m";
    pub const BRIGHT_RED: &str = "\x1b[91m";
    pub const BRIGHT_GREEN: &str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &str = "\x1b[93m";
    pub const BRIGHT_BLUE: &str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &str = "\x1b[95m";
    pub const BRIGHT_CYAN: &str = "\x1b[96m";
    pub const BRIGHT_WHITE: &str = "\x1b[97m";
    
    // Background colors
    pub const BG_BLACK: &str = "\x1b[40m";
    pub const BG_RED: &str = "\x1b[41m";
    pub const BG_GREEN: &str = "\x1b[42m";
    pub const BG_YELLOW: &str = "\x1b[43m";
    pub const BG_BLUE: &str = "\x1b[44m";
    pub const BG_MAGENTA: &str = "\x1b[45m";
    pub const BG_CYAN: &str = "\x1b[46m";
    pub const BG_WHITE: &str = "\x1b[47m";
}

pub struct TerminalRenderer {
    config: MarkdownConfig,
    theme: MarkdownTheme,
}

impl TerminalRenderer {
    pub fn new(config: &MarkdownConfig) -> Self {
        Self {
            config: config.clone(),
            theme: MarkdownTheme::default(),
        }
    }
    
    pub fn with_theme(mut self, theme: MarkdownTheme) -> Self {
        self.theme = theme;
        self
    }
    
    pub fn render(&self, document: &Document) -> Result<String, MarkdownError> {
        let mut output = String::new();
        
        for block in &document.blocks {
            self.render_block(block, &mut output)?;
            output.push('\n');
        }
        
        Ok(output)
    }
    
    fn render_block(&self, block: &Block, output: &mut String) -> Result<(), MarkdownError> {
        match block {
            Block::Heading(heading) => self.render_heading(heading, output),
            Block::Paragraph(paragraph) => self.render_paragraph(paragraph, output),
            Block::CodeBlock(code_block) => self.render_code_block(code_block, output),
            Block::List(list) => self.render_list(list, output),
            Block::Table(table) => self.render_table(table, output),
            Block::Quote(quote) => self.render_quote(quote, output),
            Block::ThematicBreak => self.render_thematic_break(output),
            Block::Html(html) => self.render_html(html, output),
        }
    }
    
    fn render_heading(&self, heading: &HeadingBlock, output: &mut String) -> Result<(), MarkdownError> {
        let color = match heading.level {
            1 => ansi::BRIGHT_BLUE,
            2 => ansi::BRIGHT_GREEN,
            3 => ansi::BRIGHT_YELLOW,
            4 => ansi::BRIGHT_MAGENTA,
            5 => ansi::BRIGHT_CYAN,
            6 => ansi::BRIGHT_WHITE,
            _ => ansi::BRIGHT_WHITE,
        };
        
        let prefix = "#".repeat(heading.level as usize);
        
        write!(output, "{}{}{} ", ansi::BOLD, color, prefix)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        for inline in &heading.content {
            self.render_inline(inline, output)?;
        }
        
        write!(output, "{}", ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        Ok(())
    }
    
    fn render_paragraph(&self, paragraph: &ParagraphBlock, output: &mut String) -> Result<(), MarkdownError> {
        for inline in &paragraph.content {
            self.render_inline(inline, output)?;
        }
        Ok(())
    }
    
    fn render_code_block(&self, code_block: &CodeBlock, output: &mut String) -> Result<(), MarkdownError> {
        let lines: Vec<&str> = code_block.code.lines().collect();
        
        // Render top border
        write!(output, "{}{}", ansi::BG_BLACK, ansi::BRIGHT_WHITE)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        if let Some(language) = &code_block.language {
            write!(output, "┌─ {} ", language)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        } else {
            write!(output, "┌─ code ")
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        }
        
        // Fill the rest of the line
        if let Some(max_width) = self.config.max_width {
            let current_len = if let Some(lang) = &code_block.language {
                7 + lang.len() // "┌─ " + language + " "
            } else {
                12 // "┌─ code "
            };
            
            if current_len < max_width {
                let padding = "─".repeat(max_width - current_len - 1);
                write!(output, "{}", padding)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
        }
        
        writeln!(output, "┐{}", ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        // Render code lines
        for (i, line) in lines.iter().enumerate() {
            write!(output, "{}{}", ansi::BG_BLACK, ansi::BRIGHT_WHITE)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            if code_block.line_numbers {
                write!(output, "│{:3} │ ", i + 1)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            } else {
                write!(output, "│ ")
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            
            // TODO: Add syntax highlighting here
            write!(output, "{}{}", ansi::BRIGHT_GREEN, line)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            // Fill the rest of the line
            if let Some(max_width) = self.config.max_width {
                let line_len = line.len() + if code_block.line_numbers { 6 } else { 2 };
                if line_len < max_width {
                    let padding = " ".repeat(max_width - line_len - 1);
                    write!(output, "{}", padding)
                        .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
                }
            }
            
            writeln!(output, "│{}", ansi::RESET)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        }
        
        // Render bottom border
        write!(output, "{}{}", ansi::BG_BLACK, ansi::BRIGHT_WHITE)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        write!(output, "└")
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        if let Some(max_width) = self.config.max_width {
            let padding = "─".repeat(max_width - 2);
            write!(output, "{}", padding)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        }
        
        writeln!(output, "┘{}", ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        Ok(())
    }
    
    fn render_list(&self, list: &ListBlock, output: &mut String) -> Result<(), MarkdownError> {
        for (i, item) in list.items.iter().enumerate() {
            let marker = if list.ordered {
                format!("{}. ", i + 1)
            } else {
                "• ".to_string()
            };
            
            write!(output, "{}{}{}", ansi::BRIGHT_CYAN, marker, ansi::RESET)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            for block in &item.content {
                self.render_block(block, output)?;
            }
            
            if i < list.items.len() - 1 {
                output.push('\n');
            }
        }
        Ok(())
    }
    
    fn render_table(&self, table: &TableBlock, output: &mut String) -> Result<(), MarkdownError> {
        if !self.config.table_rendering {
            return Ok(());
        }
        
        // Calculate column widths
        let mut col_widths = vec![0; table.headers.len()];
        
        // Check header widths
        for (i, header) in table.headers.iter().enumerate() {
            let width = self.calculate_inline_width(&header.content);
            col_widths[i] = col_widths[i].max(width);
        }
        
        // Check row widths
        for row in &table.rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let width = self.calculate_inline_width(&cell.content);
                    col_widths[i] = col_widths[i].max(width);
                }
            }
        }
        
        // Render top border
        write!(output, "{}", ansi::BRIGHT_WHITE)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        write!(output, "┌")
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        for (i, width) in col_widths.iter().enumerate() {
            write!(output, "{}", "─".repeat(width + 2))
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            if i < col_widths.len() - 1 {
                write!(output, "┬")
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
        }
        
        writeln!(output, "┐{}", ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        // Render headers
        write!(output, "{}", ansi::BRIGHT_WHITE)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        write!(output, "│")
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        for (i, header) in table.headers.iter().enumerate() {
            write!(output, " {}", ansi::BOLD)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            for inline in &header.content {
                self.render_inline(inline, output)?;
            }
            
            let current_width = self.calculate_inline_width(&header.content);
            let padding = " ".repeat(col_widths[i] - current_width + 1);
            write!(output, "{}{}", padding, ansi::BRIGHT_WHITE)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            write!(output, "│")
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        }
        
        writeln!(output, "{}", ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        // Render separator
        write!(output, "{}", ansi::BRIGHT_WHITE)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        write!(output, "├")
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        for (i, width) in col_widths.iter().enumerate() {
            write!(output, "{}", "─".repeat(width + 2))
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            if i < col_widths.len() - 1 {
                write!(output, "┼")
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
        }
        
        writeln!(output, "┤{}", ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        // Render rows
        for row in &table.rows {
            write!(output, "{}", ansi::BRIGHT_WHITE)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            write!(output, "│")
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            for (i, cell) in row.iter().enumerate() {
                write!(output, " ")
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
                
                for inline in &cell.content {
                    self.render_inline(inline, output)?;
                }
                
                let current_width = self.calculate_inline_width(&cell.content);
                let padding = " ".repeat(col_widths[i] - current_width + 1);
                write!(output, "{}{}", padding, ansi::BRIGHT_WHITE)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
                
                write!(output, "│")
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            
            writeln!(output, "{}", ansi::RESET)
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        }
        
        // Render bottom border
        write!(output, "{}", ansi::BRIGHT_WHITE)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        write!(output, "└")
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        for (i, width) in col_widths.iter().enumerate() {
            write!(output, "{}", "─".repeat(width + 2))
                .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            
            if i < col_widths.len() - 1 {
                write!(output, "┴")
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
        }
        
        writeln!(output, "┘{}", ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        Ok(())
    }
    
    fn render_quote(&self, quote: &QuoteBlock, output: &mut String) -> Result<(), MarkdownError> {
        write!(output, "{}{}", ansi::BRIGHT_BLACK, "│ ")
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        for block in &quote.content {
            self.render_block(block, output)?;
        }
        
        write!(output, "{}", ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        Ok(())
    }
    
    fn render_thematic_break(&self, output: &mut String) -> Result<(), MarkdownError> {
        let width = self.config.max_width.unwrap_or(80);
        let line = "─".repeat(width);
        
        writeln!(output, "{}{}{}", ansi::BRIGHT_BLACK, line, ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        Ok(())
    }
    
    fn render_html(&self, html: &HtmlBlock, output: &mut String) -> Result<(), MarkdownError> {
        // For now, just render as plain text with a different color
        write!(output, "{}{}{}", ansi::BRIGHT_BLACK, html.content, ansi::RESET)
            .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
        
        Ok(())
    }
    
    fn render_inline(&self, inline: &Inline, output: &mut String) -> Result<(), MarkdownError> {
        match inline {
            Inline::Text(text) => {
                write!(output, "{}", text.content)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            Inline::Emphasis(emphasis) => {
                write!(output, "{}", ansi::ITALIC)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
                
                for inline in &emphasis.content {
                    self.render_inline(inline, output)?;
                }
                
                write!(output, "{}", ansi::RESET)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            Inline::Strong(strong) => {
                write!(output, "{}", ansi::BOLD)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
                
                for inline in &strong.content {
                    self.render_inline(inline, output)?;
                }
                
                write!(output, "{}", ansi::RESET)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            Inline::Code(code) => {
                write!(output, "{}{}`{}`{}", ansi::BG_BLACK, ansi::BRIGHT_GREEN, code.content, ansi::RESET)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            Inline::Link(link) => {
                if self.config.link_highlighting {
                    write!(output, "{}{}", ansi::UNDERLINE, ansi::BRIGHT_BLUE)
                        .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
                }
                
                for inline in &link.content {
                    self.render_inline(inline, output)?;
                }
                
                if self.config.link_highlighting {
                    write!(output, "{} ({})", ansi::RESET, link.url)
                        .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
                } else {
                    write!(output, " ({})", link.url)
                        .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
                }
            }
            Inline::Image(image) => {
                write!(output, "{}{}[Image: {}] ({}){}", ansi::BRIGHT_MAGENTA, ansi::ITALIC, image.alt, image.url, ansi::RESET)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            Inline::LineBreak => {
                writeln!(output)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            Inline::SoftBreak => {
                write!(output, " ")
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
            Inline::Html(html) => {
                write!(output, "{}{}{}", ansi::BRIGHT_BLACK, html.content, ansi::RESET)
                    .map_err(|e| MarkdownError::RenderError(e.to_string()))?;
            }
        }
        
        Ok(())
    }
    
    fn calculate_inline_width(&self, inlines: &[Inline]) -> usize {
        let mut width = 0;
        
        for inline in inlines {
            match inline {
                Inline::Text(text) => width += text.content.len(),
                Inline::Emphasis(emphasis) => width += self.calculate_inline_width(&emphasis.content),
                Inline::Strong(strong) => width += self.calculate_inline_width(&strong.content),
                Inline::Code(code) => width += code.content.len() + 2, // backticks
                Inline::Link(link) => {
                    width += self.calculate_inline_width(&link.content);
                    width += link.url.len() + 3; // " ()"
                }
                Inline::Image(image) => {
                    width += image.alt.len() + image.url.len() + 12; // "[Image: ] ()"
                }
                Inline::LineBreak => width += 0,
                Inline::SoftBreak => width += 1,
                Inline::Html(html) => width += html.content.len(),
            }
        }
        
        width
    }
}
