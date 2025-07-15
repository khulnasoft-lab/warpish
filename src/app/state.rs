use crate::agent::client::AgentResponse;
use crate::agent::reasoning::ChainOfThought;
use crate::config::{Config, CursorShape, InputPosition, PromptMode, TextConfig};

// Temporary placeholder for WorkflowBrowserState
#[derive(Debug, Clone)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowBrowserState {
    pub placeholder: String,
}
use crate::drive::{DriveManager, Notebook, Workflow};
use crate::error::AppError;
use crate::event::AppEvent;
use crate::keybindings::{KeyBinding, Keymap};
use crate::pty::vte_handler::VteState;
use crate::rules::{Rule, RuleAction};
use crate::ui::theme::{Theme, ThemeManager};
use cosmic_text::{Attrs, AttrsList, Buffer, Color, Cursor, CursorMove, Editor, FontSystem, Metrics, Shaping, SwashCache, Weight, Style as FontStyle, Edit};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;
use portable_pty::{CommandBuilder, MasterPty, NativePtySystem, PtyPair, PtySize, PtySystem};
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use uuid::Uuid;
use winit::event_loop::EventLoopProxy;
use crate::completions_ui::CompletionsManager;
use crate::completions_ui::CompletionsAction;
use crate::config::EditorConfig;
use crate::vim::{VimAction, VimMode, VimMotion, VimState};
use arboard::Clipboard;
use winit::keyboard::PhysicalKey;

/// Defines the current operational mode of the application.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum AppMode {
    Normal,
    HistorySearch(HistorySearchState),
    AiPrompt,
    Agent(AgentState),
    BlockMenu(BlockMenuState),
    Settings(SettingsState),
    CommandPalette(CommandPaletteState),
    Drive(WorkflowBrowserState),
    AgentManagement,
    CodeReview(CodeReviewState),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct BlockMenuState {
    pub pane_idx: usize,
    pub block_idx: usize,
    pub selected_action_idx: usize,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct SettingsState {
    pub selected_idx: usize,
    pub filtered_list: Vec<PaletteItem>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CommandPaletteState {
    pub query: String,
    pub selected_idx: usize,
    pub filtered_list: Vec<PaletteItem>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct HistorySearchState {
    pub query: String,
    pub selected_idx: usize,
    pub filtered_list: Vec<String>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CodeReviewState {
    pub selected_file_idx: usize,
    pub selected_hunk_idx: usize,
    pub files: Vec<String>,
    pub hunks: Vec<Vec<similar::TextDiff<'static, 'static, 'static, str>>>
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum PaletteItem {
    Workflow(Workflow),
    Notebook(Notebook),
    Action { name: String, description: String, action: String },
}

pub struct App {
    pub panes: Vec<Pane>,
    pub active_pane_idx: usize,
    pub mode: AppMode,
    pub drive_manager: DriveManager,
    pub theme_manager: ThemeManager,
    pub active_theme: Theme,
    pub config: Config,
    pub autosuggestion: Option<String>,
    pub vim_state: Option<crate::vim::VimState>,
    pub input_editor: Editor<'static>,
    pub should_quit: bool,
    pub db_conn: rusqlite::Connection,
    pub undo_stack: Vec<String>,
    pub redo_stack: Vec<String>,
    pub completions_manager: CompletionsManager,
}

impl App {
    pub fn new(
        panes: Vec<Pane>,
        drive_manager: DriveManager,
        theme_manager: ThemeManager,
        theme: Theme,
        config: Config,
        db_conn: rusqlite::Connection,
        completions_manager: CompletionsManager,
    ) -> Self {
        let mut font_system = FontSystem::new();
        let metrics = Metrics::new(config.appearance.font_size, config.appearance.font_size * config.appearance.line_height);
        let mut input_editor = Editor::new(Buffer::new(&mut font_system, metrics));
        
        Self {
            panes,
            active_pane_idx: 0,
            mode: AppMode::Normal,
            drive_manager,
            theme_manager,
            active_theme: theme,
            config,
            autosuggestion: None,
            vim_state: None,
            input_editor,
            should_quit: false,
            db_conn,
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
            completions_manager,
        }
    }

    /// Queries for and updates the current autosuggestion.
    pub fn update_autosuggestion(&mut self) {
        if !self.config.editor.autosuggestions {
            self.autosuggestion = None;
            return;
        }
        
        let input_text = self.input_editor.buffer_ref().lines.iter().map(|line| line.text()).collect::<String>();
        if input_text.is_empty() {
            self.autosuggestion = None;
            return;
        }
        
        // 1. Prioritize history
        if let Ok(mut history) = crate::db::query_history_by_prefix(&mut self.db_conn, &input_text) {
            if let Some(best_match) = history.drain(..).next() {
                if best_match.len() > input_text.len() {
                    self.autosuggestion = Some(best_match[input_text.len()..].to_string());
                    return;
                }
            }
        }
        
        // 2. TODO: Fallback to completions engine
        // For now, clear if no history match
        self.autosuggestion = None;
    }

    pub fn handle_event(&mut self, event: AppEvent) -> Result<(), AppError> {
        match event {
            AppEvent::Key(key_event) => self.handle_key_event(key_event)?,
            AppEvent::Pty(data) => {
                // Handle PTY data for the active pane
                if let Some(pane) = self.panes.get_mut(self.active_pane_idx) {
                    let mut vte = pane.current_vte.lock().unwrap();
                    for byte in data {
                        vte.process(&[byte]);
                    }
                }
            }
            AppEvent::ShellExit => { self.should_quit = true; }
            AppEvent::Resize(cols, rows) => {
                for pane in &mut self.panes {
                    pane.resize(cols, rows);
                }
            }
            AppEvent::AiResult(suggestion) => {
                // Handle AI result
                if let Some(pane) = self.panes.get_mut(self.active_pane_idx) {
                    pane.pty_writer.write_all(suggestion.as_bytes())?;
                }
            }
            AppEvent::Error(e) => {
                // Log error
                log::error!("Application error: {}", e);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<(), AppError> {
        match self.mode {
            AppMode::Normal => self.handle_normal_mode_keys(key_event)?,
            AppMode::HistorySearch(_) => self.handle_history_mode_keys(key_event, &mut self.db_conn)?,
            AppMode::AiPrompt => self.handle_ai_prompt_mode_keys(key_event)?,
            _ => {}
        }
        Ok(())
    }

    fn handle_normal_mode_keys(&mut self, key: KeyEvent) -> Result<(), AppError> {
        match key.code {
            KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => self.should_quit = true,
            KeyCode::Char('r') if key.modifiers == KeyModifiers::CONTROL => self.enter_history_mode(),
            KeyCode::Char('g') if key.modifiers == KeyModifiers::CONTROL => self.enter_ai_prompt_mode(),
            _ => {
                self.handle_input(key, &mut Clipboard::new().unwrap());
            }
        }
        Ok(())
    }

    fn handle_history_mode_keys(&mut self, key: KeyEvent, db_conn: &mut rusqlite::Connection) -> Result<(), AppError> {
        if let AppMode::HistorySearch(state) = &mut self.mode {
            match key.code {
                KeyCode::Esc | KeyCode::Char('c') if key.modifiers == KeyModifiers::CONTROL => {
                    self.mode = AppMode::Normal;
                }
                KeyCode::Char(c) => {
                    state.query.push(c);
                    let all_history = crate::db::get_all_history(db_conn).unwrap_or_default();
                    let matcher = SkimMatcherV2::default();
                    state.filtered_list = all_history
                        .into_iter()
                        .filter(|item| matcher.fuzzy_match(item, &state.query).is_some())
                        .collect();
                    state.selected_idx = 0;
                }
                KeyCode::Backspace => {
                    state.query.pop();
                    let all_history = crate::db::get_all_history(db_conn).unwrap_or_default();
                    let matcher = SkimMatcherV2::default();
                    state.filtered_list = all_history
                        .into_iter()
                        .filter(|item| matcher.fuzzy_match(item, &state.query).is_some())
                        .collect();
                    state.selected_idx = 0;
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_ai_prompt_mode_keys(&mut self, key: KeyEvent) -> Result<(), AppError> {
        match key.code {
            KeyCode::Esc => self.mode = AppMode::Normal,
            _ => {}
        }
        Ok(())
    }

    fn enter_history_mode(&mut self) {
        self.mode = AppMode::HistorySearch(HistorySearchState {
            query: String::new(),
            selected_idx: 0,
            filtered_list: vec![],
        });
    }

    fn enter_ai_prompt_mode(&mut self) {
        self.mode = AppMode::AiPrompt;
    }

    /// Top-level input dispatcher.
    pub fn handle_input(&mut self, key: KeyEvent, clipboard: &mut Clipboard) -> Option<String> {
        let text_before = self.input_editor.buffer_ref().lines.iter().map(|line| line.text()).collect::<String>();
        let mut text_changed = false;

        let result = if let Some(state) = &mut self.vim_state {
            self.handle_vim_input(state, &key, clipboard, &mut text_changed)
        } else {
            self.handle_modern_input(&key, clipboard, &mut text_changed)
        };

        if text_changed {
            let text_after = self.input_editor.buffer_ref().lines.iter().map(|line| line.text()).collect::<String>();
            if text_before != text_after {
                self.undo_stack.push(text_before);
                self.redo_stack.clear();
            }
        }

        result
    }

    /// The existing modern input handler, renamed.
    pub fn handle_modern_input(&mut self, key: &KeyEvent, clipboard: &mut Clipboard, text_changed: &mut bool) -> Option<String> {
        if key.state != winit::event::ElementState::Pressed {
            return None;
        }

        let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);
        let alt = key.modifiers.contains(KeyModifiers::ALT);
        let shift = key.modifiers.contains(KeyModifiers::SHIFT);
        let super_key = key.modifiers.contains(KeyModifiers::SUPER); // CMD on macOS

        // --- Handle completions first ---
        if self.completions_manager.ui.is_visible {
            let completions_action = self.completions_manager.handle_key_event(key.physical_key);
            match completions_action {
                CompletionsAction::Accept(replacement) => {
                    // Replace the current word with the selected suggestion
                    let current_text = self.input_editor.buffer_ref().lines.iter().map(|line| line.text()).collect::<String>();
                    let cursor_pos = self.input_editor.buffer_ref().cursor().index;
                    
                    // Find the current word boundaries
                    if let Some((word_start, word_end)) = find_word_boundaries(&current_text, cursor_pos) {
                        let before_word = &current_text[..word_start];
                        let after_word = &current_text[word_end..];
                        let new_text = format!("{}{}{}", before_word, replacement, after_word);
                        
                        // Update the editor
                        self.input_editor.buffer_ref_mut().set_text(&mut self.input_editor.font_system, &new_text, AttrsList::new(Attrs::new()), Shaping::Advanced);
                        self.input_editor.set_cursor(Cursor::new(word_start, 0)); // Assuming 0 is the line number
                    }
                    
                    self.completions_manager.ui.hide();
                    *text_changed = true;
                    return None;
                }
                CompletionsAction::Close => {
                    self.completions_manager.ui.hide();
                    return None;
                }
                CompletionsAction::Navigate => {
                    return None; // UI will be updated by the renderer
                }
                CompletionsAction::None => {
                    // Continue with normal input handling
                }
            }
        }

        // --- High-priority actions (copy, paste, cut) ---
        if super_key && key.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyC) {
            self.input_editor.copy_selection();
            return None;
        }
        if super_key && key.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyX) {
            self.input_editor.cut_selection();
            return None;
        }
        if super_key && key.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyV) {
            if let Ok(text) = clipboard.get_text() {
                self.input_editor.insert_string(text, None);
            }
            return None;
        }

        // --- Main keybinding logic ---
        let action = match (key.physical_key, ctrl, alt, shift) {
            // --- Cursor Movement ---
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft), false, false, false) => CursorMove::Left,
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowRight), false, false, false) => CursorMove::Right,
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowUp), false, false, false) => CursorMove::Up,
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowDown), false, false, false) => CursorMove::Down,
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft), false, true, false) => CursorMove::WordLeft,
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowRight), false, true, false) => CursorMove::WordRight,
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA), true, _, _) | (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Home), _, _, _) => CursorMove::LineStart,
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyE), true, _, _) | (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::End), _, _, _) => CursorMove::LineEnd,

            // --- Deletion ---
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Backspace), false, false, false) => {
                self.input_editor.delete_char_back();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Delete), false, false, false) => {
                self.input_editor.delete_char_forward();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Backspace), false, true, false) => {
                self.input_editor.delete_word_back();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyD), false, true, false) => {
                self.input_editor.delete_word_forward();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyK), true, _, _) => {
                self.input_editor.delete_to_line_end();
                *text_changed = true;
                return None;
            }

            // --- Selection ---
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA), true, _, true) | (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::KeyA), _, _, _,) if super_key => {
                self.input_editor.select_all();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft), false, false, true) => {
                self.input_editor.select_left();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowRight), false, false, true) => {
                self.input_editor.select_right();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowLeft), false, true, true) => {
                self.input_editor.select_word_left();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::ArrowRight), false, true, true) => {
                self.input_editor.select_word_right();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Home), false, false, true) => {
                self.input_editor.select_line_start();
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::End), false, false, true) => {
                self.input_editor.select_line_end();
                *text_changed = true;
                return None;
            }
            
            // --- Enter / Newline ---
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Enter), _, _, true) => {
                self.input_editor.insert_string("\n", None);
                *text_changed = true;
                return None;
            }
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Enter), false, false, false) => {
                let command = self.input_editor.buffer_ref().lines.iter().map(|line| line.text()).collect::<Vec<_>>().join("\n");
                self.input_editor.buffer_ref_mut().set_text(&mut self.input_editor.font_system, "", AttrsList::new(Attrs::new()), Shaping::Advanced); // Clears the editor
                
                // Add command to completions history
                if !command.trim().is_empty() {
                    self.completions_manager.add_to_history(command.clone());
                }
                
                return Some(command);
            }
            
            // --- Tab for completions ---
            (winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Tab), false, false, false) => {
                if self.completions_manager.ui.is_visible {
                    // Handle tab completion
                    let completions_action = self.completions_manager.handle_key_event(winit::keyboard::KeyCode::Tab);
                    if let CompletionsAction::Accept(replacement) = completions_action {
                        // Replace current word with suggestion
                        let current_text = self.input_editor.buffer_ref().lines.iter().map(|line| line.text()).collect::<String>();
                        let cursor_pos = self.input_editor.buffer_ref().cursor().index;
                        
                        if let Some((word_start, word_end)) = find_word_boundaries(&current_text, cursor_pos) {
                            let before_word = &current_text[..word_start];
                            let after_word = &current_text[word_end..];
                            let new_text = format!("{}{}{}", before_word, replacement, after_word);
                            
                            self.input_editor.buffer_ref_mut().set_text(&mut self.input_editor.font_system, &new_text, AttrsList::new(Attrs::new()), Shaping::Advanced);
                            self.input_editor.set_cursor(Cursor::new(word_start, 0));
                        }
                        
                        self.completions_manager.ui.hide();
                        *text_changed = true;
                    }
                    return None;
                }
                self.input_editor.insert_string("\t", None);
                *text_changed = true;
                return None;
            }
            
            // --- Text Input ---
            _ => {
                if let Some(text) = &key.text {
                    if self.config.editor.autocomplete_pairs {
                        let pair = match text.as_str() {
                            "(" => Some(")"),
                            "[" => Some("]"),
                            "{" => Some("}"),
                            "\"" => Some("\""),
                            "'" => Some("'"),
                            _ => None,
                        };
                        if let Some(closing) = pair {
                            self.input_editor.insert_string(text.to_string(), None);
                            self.input_editor.insert_string(closing.to_string(), None);
                            self.input_editor.move_cursor(CursorMove::Left);
                        } else {
                            self.input_editor.insert_string(text.to_string(), None);
                        }
                    } else {
                        self.input_editor.insert_string(text.to_string(), None);
                    }
                }
                *text_changed = true;
                return None;
            }
        };
        
        // If we reach here, it means a CursorMove action was returned
        self.input_editor.move_cursor(action);
        *text_changed = true;
        None
    }

    /// New handler for Vim mode.
    fn handle_vim_input(&mut self, state: &mut VimState, key: &KeyEvent, clipboard: &mut Clipboard, text_changed: &mut bool) -> Option<String> {
        // --- Parse the key into a high-level Vim action ---
        let vim_action = state.handle_key(key);
        // --- Execute the Vim action using cosmic-text actions ---
        match vim_action {
            VimAction::EnterInsertMode => state.mode = VimMode::Insert,
            VimAction::EnterNormalMode => state.mode = VimMode::Normal,
            VimAction::EnterVisualMode => {
                state.mode = VimMode::Visual;
                self.input_editor.select_all(); // Simplified for now
            }
            VimAction::Move(motion) => {
                let action = match motion {
                    VimMotion::Left => CursorMove::Left,
                    VimMotion::Right => CursorMove::Right,
                    VimMotion::Up => CursorMove::Up,
                    VimMotion::Down => CursorMove::Down,
                    VimMotion::WordForward => CursorMove::WordRight,
                    VimMotion::WordBackward => CursorMove::WordLeft,
                    VimMotion::LineStart => CursorMove::LineStart,
                    VimMotion::LineEnd => CursorMove::LineEnd,
                };
                self.input_editor.move_cursor(action);
            }
            VimAction::Delete(_motion) => {
                self.input_editor.delete_word_forward(); // Simplified example
            }
            VimAction::Paste => { if let Ok(text) = clipboard.get_text() { self.input_editor.insert_string(text, None); } }
            VimAction::Undo => {
                if let Some(text) = self.undo_stack.pop() {
                    let current_text = self.input_editor.buffer_ref().lines.iter().map(|line| line.text()).collect::<String>();
                    self.redo_stack.push(current_text);
                    self.input_editor.buffer_ref_mut().set_text(&mut self.input_editor.font_system, &text, AttrsList::new(Attrs::new()), Shaping::Advanced);
                }
            }
            _ => {}
        }
        // If we're in Insert mode, handle text input like normal
        if state.mode == VimMode::Insert {
            if let Some(text) = &key.text {
                self.input_editor.insert_string(text.to_string(), None);
            }
            if key.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Backspace) {
                self.input_editor.delete_char_back();
            }
        }
        // In Vim mode, Enter only executes from Normal mode (simplified)
        if state.mode == VimMode::Normal && key.physical_key == winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::Enter) {
            let command = self.input_editor.buffer_ref().lines.iter().map(|line| line.text()).collect::<Vec<_>>().join("\n");
            self.input_editor.buffer_ref_mut().set_text(&mut self.input_editor.font_system, "", AttrsList::new(Attrs::new()), Shaping::Advanced);
            
            // Add command to completions history
            if !command.trim().is_empty() {
                self.completions_manager.add_to_history(command.clone());
            }
            
            return Some(command);
        }
        *text_changed = true;
        None
    }
}

/// Helper function to find word boundaries for completion replacement
fn find_word_boundaries(text: &str, cursor_pos: usize) -> Option<(usize, usize)> {
    if cursor_pos > text.len() {
        return None;
    }
    
    let mut start = cursor_pos;
    let mut end = cursor_pos;
    
    // Find word start
    while start > 0 {
        let prev_char = text.chars().nth(start - 1)?;
        if prev_char.is_whitespace() {
            break;
        }
        start -= 1;
    }
    
    // Find word end
    while end < text.len() {
        let next_char = text.chars().nth(end)?;
        if next_char.is_whitespace() {
            break;
        }
        end += 1;
    }
    
    Some((start, end))
}

// Legacy types that might be needed for compatibility
pub type Database = ();
pub type FuzzyFinder = ();
pub type CommandHistory = ();
pub type Parser = ();
pub type Grid = ();
pub type AppResult<T> = Result<T, AppError>;