//! Terminal UI Module
//! 
//! This module provides a terminal user interface with Blocks functionality
//! using crossterm and ratatui for rendering.

use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span, Text},
    widgets::{
        Block as UIBlock, BorderType, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap,
    },
    Frame, Terminal,
};
use std::io::{self, stdout, Stdout};
use std::time::{Duration, Instant};

use super::blocks::{Block, BlockManager, CommandStatus};

pub struct TerminalUI {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    block_manager: BlockManager,
    list_state: ListState,
    input_buffer: String,
    mode: UIMode,
    scroll_offset: usize,
    search_query: String,
    show_help: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UIMode {
    Normal,
    Input,
    Search,
    BlockNavigation,
    Help,
}

impl TerminalUI {
    pub fn new() -> Result<Self, io::Error> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;

        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Ok(Self {
            terminal,
            block_manager: BlockManager::default(),
            list_state,
            input_buffer: String::new(),
            mode: UIMode::Normal,
            scroll_offset: 0,
            search_query: String::new(),
            show_help: false,
        })
    }

    pub fn run(&mut self) -> Result<(), io::Error> {
        let mut last_tick = Instant::now();
        let tick_rate = Duration::from_millis(250);

        loop {
            self.terminal.draw(|f| self.ui(f))?;

            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if crossterm::event::poll(timeout)? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        match self.handle_key(key.code) {
                            Ok(should_quit) => {
                                if should_quit {
                                    break;
                                }
                            }
                            Err(_) => break,
                        }
                    }
                }
            }

            if last_tick.elapsed() >= tick_rate {
                last_tick = Instant::now();
            }
        }

        Ok(())
    }

    fn handle_key(&mut self, key: KeyCode) -> Result<bool, io::Error> {
        match self.mode {
            UIMode::Normal => self.handle_normal_key(key),
            UIMode::Input => self.handle_input_key(key),
            UIMode::Search => self.handle_search_key(key),
            UIMode::BlockNavigation => self.handle_navigation_key(key),
            UIMode::Help => self.handle_help_key(key),
        }
    }

    fn handle_normal_key(&mut self, key: KeyCode) -> Result<bool, io::Error> {
        match key {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('i') => self.mode = UIMode::Input,
            KeyCode::Char('/') => {
                self.mode = UIMode::Search;
                self.search_query.clear();
            }
            KeyCode::Char('n') => self.mode = UIMode::BlockNavigation,
            KeyCode::Char('h') | KeyCode::F1 => self.show_help = !self.show_help,
            KeyCode::Char('c') => self.copy_current_block_command(),
            KeyCode::Char('o') => self.copy_current_block_output(),
            KeyCode::Char('b') => self.copy_current_block_both(),
            KeyCode::Char('s') => self.share_current_block(),
            KeyCode::Char('m') => self.toggle_current_block_bookmark(),
            KeyCode::Char('d') => self.delete_current_block(),
            KeyCode::Up => self.select_previous_block(),
            KeyCode::Down => self.select_next_block(),
            KeyCode::PageUp => self.scroll_up(),
            KeyCode::PageDown => self.scroll_down(),
            KeyCode::Enter => self.execute_current_command(),
            _ => {}
        }
        Ok(false)
    }

    fn handle_input_key(&mut self, key: KeyCode) -> Result<bool, io::Error> {
        match key {
            KeyCode::Escape => self.mode = UIMode::Normal,
            KeyCode::Enter => {
                self.execute_command();
                self.mode = UIMode::Normal;
            }
            KeyCode::Backspace => {
                self.input_buffer.pop();
            }
            KeyCode::Char(c) => {
                self.input_buffer.push(c);
            }
            _ => {}
        }
        Ok(false)
    }

    fn handle_search_key(&mut self, key: KeyCode) -> Result<bool, io::Error> {
        match key {
            KeyCode::Escape => self.mode = UIMode::Normal,
            KeyCode::Enter => {
                self.perform_search();
                self.mode = UIMode::Normal;
            }
            KeyCode::Backspace => {
                self.search_query.pop();
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
            }
            _ => {}
        }
        Ok(false)
    }

    fn handle_navigation_key(&mut self, key: KeyCode) -> Result<bool, io::Error> {
        match key {
            KeyCode::Escape => self.mode = UIMode::Normal,
            KeyCode::Up => self.select_previous_block(),
            KeyCode::Down => self.select_next_block(),
            KeyCode::Enter => {
                self.navigate_to_selected_block();
                self.mode = UIMode::Normal;
            }
            _ => {}
        }
        Ok(false)
    }

    fn handle_help_key(&mut self, key: KeyCode) -> Result<bool, io::Error> {
        match key {
            KeyCode::Escape | KeyCode::Char('h') | KeyCode::F1 => {
                self.show_help = false;
                self.mode = UIMode::Normal;
            }
            _ => {}
        }
        Ok(false)
    }

    fn ui(&mut self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(0),    // Main content
                Constraint::Length(3), // Footer/Input
            ])
            .split(f.size());

        self.render_header(f, chunks[0]);
        self.render_main_content(f, chunks[1]);
        self.render_footer(f, chunks[2]);

        if self.show_help {
            self.render_help(f);
        }
    }

    fn render_header(&self, f: &mut Frame, area: Rect) {
        let stats = self.block_manager.get_statistics();
        let title = format!(
            " Warpish Terminal - {} blocks ({} successful, {} bookmarked) ",
            stats.total_blocks, stats.successful_blocks, stats.bookmarked_blocks
        );

        let header = Paragraph::new(title)
            .style(Style::default().fg(Color::Cyan))
            .alignment(Alignment::Center)
            .block(
                UIBlock::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Cyan)),
            );

        f.render_widget(header, area);
    }

    fn render_main_content(&mut self, f: &mut Frame, area: Rect) {
        let blocks = self.block_manager.get_all_blocks();
        
        if blocks.is_empty() {
            let empty_msg = Paragraph::new("No blocks yet. Press 'i' to enter a command.")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center)
                .block(UIBlock::default().borders(Borders::ALL));
            f.render_widget(empty_msg, area);
            return;
        }

        let items: Vec<ListItem> = blocks
            .iter()
            .enumerate()
            .map(|(i, block)| {
                let status_symbol = match block.status {
                    CommandStatus::Success => "✅",
                    CommandStatus::Error(_) => "❌",
                    CommandStatus::Running => "⏳",
                    CommandStatus::Cancelled => "⏹️",
                };

                let bookmark_symbol = if block.bookmarked { "🔖" } else { "  " };
                
                let command_style = match block.status {
                    CommandStatus::Success => Style::default().fg(Color::Green),
                    CommandStatus::Error(_) => Style::default().fg(Color::Red),
                    CommandStatus::Running => Style::default().fg(Color::Yellow),
                    CommandStatus::Cancelled => Style::default().fg(Color::Gray),
                };

                let line = Line::from(vec![
                    Span::raw(format!("{:3} ", i + 1)),
                    Span::raw(status_symbol),
                    Span::raw(" "),
                    Span::raw(bookmark_symbol),
                    Span::raw(" "),
                    Span::styled(block.command.clone(), command_style),
                ]);

                ListItem::new(line)
            })
            .collect();

        let list = List::new(items)
            .block(
                UIBlock::default()
                    .borders(Borders::ALL)
                    .title("Blocks")
                    .border_type(BorderType::Rounded),
            )
            .highlight_style(Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD))
            .highlight_symbol(">> ");

        f.render_stateful_widget(list, area, &mut self.list_state);
    }

    fn render_footer(&self, f: &mut Frame, area: Rect) {
        let footer_text = match self.mode {
            UIMode::Normal => "q: quit | i: input | /: search | n: navigate | h: help | c: copy cmd | o: copy output | b: copy both | s: share | m: bookmark",
            UIMode::Input => &format!("Enter command: {} | ESC: cancel", self.input_buffer),
            UIMode::Search => &format!("Search: {} | ESC: cancel", self.search_query),
            UIMode::BlockNavigation => "Navigate blocks: ↑/↓ to select | Enter: go to | ESC: cancel",
            UIMode::Help => "Press ESC or h to close help",
        };

        let footer = Paragraph::new(footer_text)
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .block(
                UIBlock::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::White)),
            );

        f.render_widget(footer, area);
    }

    fn render_help(&self, f: &mut Frame) {
        let help_text = vec![
            "Warpish Terminal - Blocks Help",
            "",
            "Navigation:",
            "  ↑/↓        - Navigate blocks",
            "  Page Up/Down - Scroll output",
            "  Enter      - Execute selected command",
            "",
            "Commands:",
            "  q          - Quit",
            "  i          - Enter command input mode",
            "  /          - Search blocks",
            "  n          - Block navigation mode",
            "  h / F1     - Toggle this help",
            "",
            "Block Operations:",
            "  c          - Copy command",
            "  o          - Copy output",
            "  b          - Copy both",
            "  s          - Share block",
            "  m          - Toggle bookmark",
            "  d          - Delete block",
            "",
            "Press ESC or h to close this help.",
        ];

        let help_paragraph = Paragraph::new(help_text.join("\n"))
            .style(Style::default().fg(Color::White))
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .block(
                UIBlock::default()
                    .borders(Borders::ALL)
                    .title("Help")
                    .border_type(BorderType::Rounded)
                    .style(Style::default().fg(Color::Cyan)),
            );

        let area = centered_rect(60, 70, f.size());
        f.render_widget(Clear, area);
        f.render_widget(help_paragraph, area);
    }

    // Block operations
    fn execute_command(&mut self) {
        if !self.input_buffer.is_empty() {
            let command = self.input_buffer.clone();
            let cwd = std::env::current_dir()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            let block = self.block_manager.create_block(command, cwd);
            
            // Simulate command execution (in real implementation, this would be async)
            block.set_output("Command executed successfully".to_string());
            block.set_status(CommandStatus::Success);
            block.set_execution_time(Duration::from_millis(100));

            self.input_buffer.clear();
            self.list_state.select(Some(self.block_manager.get_all_blocks().len() - 1));
        }
    }

    fn execute_current_command(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let blocks = self.block_manager.get_all_blocks();
            if let Some(block) = blocks.get(selected) {
                self.input_buffer = block.command.clone();
                self.mode = UIMode::Input;
            }
        }
    }

    fn copy_current_block_command(&self) {
        if let Some(selected) = self.list_state.selected() {
            let blocks = self.block_manager.get_all_blocks();
            if let Some(block) = blocks.get(selected) {
                // In real implementation, copy to clipboard
                println!("Copied command: {}", block.copy_command());
            }
        }
    }

    fn copy_current_block_output(&self) {
        if let Some(selected) = self.list_state.selected() {
            let blocks = self.block_manager.get_all_blocks();
            if let Some(block) = blocks.get(selected) {
                // In real implementation, copy to clipboard
                println!("Copied output: {}", block.copy_output());
            }
        }
    }

    fn copy_current_block_both(&self) {
        if let Some(selected) = self.list_state.selected() {
            let blocks = self.block_manager.get_all_blocks();
            if let Some(block) = blocks.get(selected) {
                // In real implementation, copy to clipboard
                println!("Copied both: {}", block.copy_both());
            }
        }
    }

    fn share_current_block(&self) {
        if let Some(selected) = self.list_state.selected() {
            let blocks = self.block_manager.get_all_blocks();
            if let Some(block) = blocks.get(selected) {
                // In real implementation, share via system sharing
                println!("Shared: {}", block.format_for_sharing());
            }
        }
    }

    fn toggle_current_block_bookmark(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let blocks = self.block_manager.get_all_blocks();
            if let Some(block) = blocks.get(selected) {
                let block_id = block.id.clone();
                if let Some(block) = self.block_manager.get_block_by_id_mut(&block_id) {
                    block.toggle_bookmark();
                }
            }
        }
    }

    fn delete_current_block(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let blocks = self.block_manager.get_all_blocks();
            if let Some(block) = blocks.get(selected) {
                let block_id = block.id.clone();
                self.block_manager.delete_block(&block_id);
                
                // Adjust selection
                let new_len = self.block_manager.get_all_blocks().len();
                if new_len > 0 {
                    let new_selected = if selected >= new_len { new_len - 1 } else { selected };
                    self.list_state.select(Some(new_selected));
                } else {
                    self.list_state.select(None);
                }
            }
        }
    }

    fn select_previous_block(&mut self) {
        let blocks_len = self.block_manager.get_all_blocks().len();
        if blocks_len > 0 {
            let selected = self.list_state.selected().unwrap_or(0);
            let new_selected = if selected > 0 { selected - 1 } else { blocks_len - 1 };
            self.list_state.select(Some(new_selected));
        }
    }

    fn select_next_block(&mut self) {
        let blocks_len = self.block_manager.get_all_blocks().len();
        if blocks_len > 0 {
            let selected = self.list_state.selected().unwrap_or(0);
            let new_selected = if selected < blocks_len - 1 { selected + 1 } else { 0 };
            self.list_state.select(Some(new_selected));
        }
    }

    fn scroll_up(&mut self) {
        if self.scroll_offset > 0 {
            self.scroll_offset -= 1;
        }
    }

    fn scroll_down(&mut self) {
        self.scroll_offset += 1;
    }

    fn perform_search(&mut self) {
        let results = self.block_manager.search_blocks(&self.search_query);
        if !results.is_empty() {
            // In real implementation, filter the list to show only search results
            println!("Search results: {} blocks found", results.len());
        }
    }

    fn navigate_to_selected_block(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            let blocks = self.block_manager.get_all_blocks();
            if let Some(block) = blocks.get(selected) {
                self.block_manager.navigate_to_block(&block.id);
            }
        }
    }
}

impl Drop for TerminalUI {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
        let _ = execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        );
        let _ = self.terminal.show_cursor();
    }
}

// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
