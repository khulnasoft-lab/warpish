use crate::completions::{CompletionManager, Suggestion, SuggestionType};
use cosmic_text::{Attrs, Buffer, Color, Editor, FontSystem, Metrics, Shaping};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct CompletionsUI {
    pub suggestions: Vec<Suggestion>,
    pub selected_index: usize,
    pub is_visible: bool,
    pub position: (f32, f32),
    pub max_width: f32,
    pub max_height: f32,
}

impl CompletionsUI {
    pub fn new() -> Self {
        Self {
            suggestions: Vec::new(),
            selected_index: 0,
            is_visible: false,
            position: (0.0, 0.0),
            max_width: 600.0,
            max_height: 400.0,
        }
    }

    pub fn show(&mut self, suggestions: Vec<Suggestion>, position: (f32, f32)) {
        self.suggestions = suggestions;
        self.selected_index = 0;
        self.is_visible = true;
        self.position = position;
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
        self.suggestions.clear();
    }

    pub fn next(&mut self) {
        if !self.suggestions.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.suggestions.len();
        }
    }

    pub fn previous(&mut self) {
        if !self.suggestions.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.suggestions.len() - 1
            } else {
                self.selected_index - 1
            };
        }
    }

    pub fn get_selected_suggestion(&self) -> Option<&Suggestion> {
        self.suggestions.get(self.selected_index)
    }

    pub fn render(&self, font_system: &mut FontSystem, metrics: Metrics) -> Buffer {
        if !self.is_visible || self.suggestions.is_empty() {
            return Buffer::new(font_system, metrics);
        }

        let mut buffer = Buffer::new(font_system, metrics);
        buffer.set_size(font_system, self.max_width, self.max_height);

        let mut text = String::new();
        let mut attrs_list = cosmic_text::AttrsList::new(Attrs::new());

        // Header
        text.push_str("Completions\n");
        text.push_str(&"=".repeat(20));
        text.push('\n');

        // Suggestions
        for (i, suggestion) in self.suggestions.iter().enumerate() {
            let is_selected = i == self.selected_index;
            let prefix = if is_selected { "> " } else { "  " };
            
            // Main suggestion text
            let line_start = text.len();
            text.push_str(prefix);
            text.push_str(&suggestion.display);
            
            // Add description if available
            if let Some(desc) = &suggestion.description {
                text.push_str(" - ");
                text.push_str(desc);
            }
            
            // Add type indicator
            let type_indicator = match suggestion.suggestion_type {
                SuggestionType::Command => "[CMD]",
                SuggestionType::Subcommand => "[SUB]",
                SuggestionType::Flag => "[FLAG]",
                SuggestionType::Argument => "[ARG]",
                SuggestionType::FilePath => "[FILE]",
                SuggestionType::History => "[HIST]",
                SuggestionType::AiGenerated => "[AI]",
                SuggestionType::Workflow => "[WF]",
            };
            text.push_str(" ");
            text.push_str(type_indicator);
            
            text.push('\n');

            // Apply styling
            let line_end = text.len();
            let attrs = if is_selected {
                Attrs::new()
                    .color(Color::rgb(255, 255, 255))
                    .weight(cosmic_text::Weight::BOLD)
            } else {
                Attrs::new()
                    .color(Color::rgb(200, 200, 200))
            };
            attrs_list.add_span(line_start..line_end, attrs);
        }

        // Footer with navigation hints
        text.push_str("\n");
        text.push_str("↑/↓: Navigate  Enter: Select  Tab: Accept  Esc: Close");

        buffer.set_text(font_system, &text, attrs_list, Shaping::Advanced);
        buffer
    }
}

/// Manager for handling completions in the main application
#[derive(Clone)]
pub struct CompletionsManager {
    pub completion_manager: Arc<Mutex<CompletionManager>>,
    pub ui: CompletionsUI,
    pub is_enabled: bool,
    pub trigger_chars: Vec<char>,
    pub min_trigger_length: usize,
}

impl CompletionsManager {
    pub fn new() -> Self {
        Self {
            completion_manager: Arc::new(Mutex::new(CompletionManager::new())),
            ui: CompletionsUI::new(),
            is_enabled: true,
            trigger_chars: vec![' ', '\t', '/', '-', '.'],
            min_trigger_length: 1,
        }
    }

    pub fn should_trigger_completion(&self, current_text: &str, cursor_pos: usize) -> bool {
        if !self.is_enabled || current_text.len() < self.min_trigger_length {
            return false;
        }

        // Check if we're at a word boundary or after a trigger character
        if cursor_pos == 0 {
            return true;
        }

        let char_before_cursor = current_text.chars().nth(cursor_pos - 1);
        if let Some(ch) = char_before_cursor {
            return self.trigger_chars.contains(&ch);
        }

        false
    }

    pub async fn update_suggestions(&mut self, current_text: &str, cursor_pos: usize) {
        if !self.should_trigger_completion(current_text, cursor_pos) {
            self.ui.hide();
            return;
        }

        let completion_manager = self.completion_manager.clone();
        let suggestions = completion_manager.lock().await.get_all_suggestions(current_text, cursor_pos).await;

        if suggestions.is_empty() {
            self.ui.hide();
        } else {
            // Calculate position for the completions UI
            // This would typically be below the cursor
            let position = (50.0, 100.0); // Placeholder position
            self.ui.show(suggestions, position);
        }
    }

    pub fn handle_key_event(&mut self, key_code: winit::keyboard::KeyCode) -> CompletionsAction {
        if !self.ui.is_visible {
            return CompletionsAction::None;
        }

        match key_code {
            winit::keyboard::KeyCode::ArrowDown => {
                self.ui.next();
                CompletionsAction::Navigate
            }
            winit::keyboard::KeyCode::ArrowUp => {
                self.ui.previous();
                CompletionsAction::Navigate
            }
            winit::keyboard::KeyCode::Enter => {
                if let Some(suggestion) = self.ui.get_selected_suggestion() {
                    CompletionsAction::Accept(suggestion.replacement.clone())
                } else {
                    CompletionsAction::None
                }
            }
            winit::keyboard::KeyCode::Tab => {
                if let Some(suggestion) = self.ui.get_selected_suggestion() {
                    CompletionsAction::Accept(suggestion.replacement.clone())
                } else {
                    CompletionsAction::None
                }
            }
            winit::keyboard::KeyCode::Escape => {
                self.ui.hide();
                CompletionsAction::Close
            }
            _ => CompletionsAction::None,
        }
    }

    pub fn add_to_history(&self, command: String) {
        let completion_manager = self.completion_manager.clone();
        tokio::spawn(async move {
            completion_manager.lock().await.add_to_history(command);
        });
    }
}

#[derive(Debug, Clone)]
pub enum CompletionsAction {
    None,
    Navigate,
    Accept(String),
    Close,
} 