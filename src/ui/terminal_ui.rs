//! Terminal UI Module
//! 
//! This module provides a terminal user interface with Blocks functionality
//! using crossterm and ratatui for rendering.

use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
    Frame, Terminal,
};
use std::io::{self, stdout, Stdout};
use std::time::{Duration, Instant};

use crate::app::{
    state::{App},
    pane_manager::BlockManager,
};
use crate::config::theme::Theme;
use std::sync::{Arc, Mutex};

// FIX: TerminalUI is now stateless. All state is passed in from App.
pub struct TerminalUI;

impl TerminalUI {
    pub fn new() -> Self {
        Self
    }
    
    /// Convert hex color string to ratatui Color
    fn hex_to_ratatui_color(&self, hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            return Color::White; // Default fallback
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
        
        Color::Rgb(r, g, b)
    }

    pub fn draw<B: Backend>(&self, terminal: &mut Terminal<B>, app: &mut App) -> std::io::Result<()> {
        terminal.draw(|f| self.ui(f, app))?;
        Ok(())
    }

    // FIX: Takes a mutable reference to app state to draw the UI
    fn ui(&self, f: &mut Frame, app: &mut App) {
        // Get theme colors for UI styling
        let theme = &app.theme;
        let bg_color = self.hex_to_ratatui_color(&theme.background);
        let fg_color = self.hex_to_ratatui_color(&theme.foreground);
        let highlight_color = self.hex_to_ratatui_color(&theme.terminal_colors.normal.blue);
        
        // ... rendering logic now uses `app.mode`, `app.block_manager`, etc.
        // Example for rendering blocks:
        let block_manager = app.block_manager.lock().unwrap();
        let blocks = block_manager.get_all_blocks();
        let items: Vec<ListItem> = blocks
            .iter()
            .map(|block| {
                ListItem::new(block.command.clone())
                    .style(Style::default().fg(fg_color))
            })
            .collect();
        
        let list = List::new(items)
            .block(Block::default()
                .title("Blocks")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(fg_color))
                .style(Style::default().bg(bg_color)))
            .highlight_style(Style::default()
                .add_modifier(Modifier::BOLD)
                .fg(highlight_color))
            .highlight_symbol("> ");

        // We need a ListState, which should now live in the App struct
        // For example: `f.render_stateful_widget(list, area, &mut app.list_state);`
    }
}
