use crate::completions::{CompletionManager, Suggestion};
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping};
use std::sync::Arc;
use tokio::sync::Mutex;

// FIX: Added lifetime 'a for Attrs and derived Clone
#[derive(Clone)]
pub struct CompletionsUI<'a> {
    pub suggestions: Vec<Suggestion>,
    pub selected_index: usize,
    pub buffer: Option<Buffer>,
    pub metrics: Metrics,
    pub attrs: Attrs<'a>,
    pub max_width: f32,
    pub max_height: f32,
    // FIX: Added missing field
    pub is_visible: bool,
}

impl<'a> CompletionsUI<'a> {
    // FIX: Corrected signature and initialization
    pub fn new(font_size: f32, line_height: f32) -> Self {
        Self {
            suggestions: Vec::new(),
            selected_index: 0,
            buffer: None,
            // FIX: Correctly call Metrics::new
            metrics: Metrics::new(font_size, line_height),
            attrs: Attrs::new(),
            max_width: 400.0,
            max_height: 200.0,
            is_visible: false,
        }
    }

    pub fn update_suggestions(&mut self, completion_manager: &CompletionManager, input: &str) {
        // FIX: Removed .await as the method is not async
        self.suggestions = completion_manager.get_all_suggestions(input);
        self.is_visible = !self.suggestions.is_empty();
    }

    pub fn update_buffer(&mut self, font_system: &mut FontSystem) {
        let mut buffer = Buffer::new(font_system, self.metrics);
        buffer.set_size(font_system, Some(self.max_width), Some(self.max_height));

        let mut text = String::new();
        for (i, suggestion) in self.suggestions.iter().enumerate() {
            let line = if i == self.selected_index {
                format!("> {}\n", suggestion.display)
            } else {
                format!("  {}\n", suggestion.display)
            };
            text.push_str(&line);
        }

        // FIX: set_text expects Attrs, not AttrsList
        buffer.set_text(font_system, &text, self.attrs, Shaping::Advanced);
        self.buffer = Some(buffer);
    }
}

/// Manager for handling completions in the main application
#[derive(Clone)]
pub struct CompletionsManager {
    pub completion_manager: Arc<Mutex<CompletionManager>>,
    pub ui: CompletionsUI<'static>,
    pub is_enabled: bool,
    pub trigger_chars: Vec<char>,
    pub min_trigger_length: usize,
}

impl CompletionsManager {
    pub fn new() -> Self {
        Self {
            completion_manager: Arc::new(Mutex::new(CompletionManager::new())),
            ui: CompletionsUI::new(16.0, 24.0), // Initialize with default font size and line height
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
            self.ui.is_visible = false;
            return;
        }

        let completion_manager = self.completion_manager.clone();
        let suggestions = completion_manager.lock().await.get_all_suggestions(current_text);

        if suggestions.is_empty() {
            self.ui.is_visible = false;
        } else {
            // Calculate position for the completions UI
            // This would typically be below the cursor
            let position = (50.0, 100.0); // Placeholder position
            let cm = completion_manager.lock().await;
            self.ui.update_suggestions(&*cm, current_text);
            self.ui.is_visible = true;
        }
    }

    pub fn handle_key_event(&mut self, key_code: winit::keyboard::KeyCode) -> CompletionsAction {
        if !self.ui.is_visible {
            return CompletionsAction::None;
        }

        match key_code {
            winit::keyboard::KeyCode::ArrowDown => {
                self.ui.selected_index = (self.ui.selected_index + 1) % self.ui.suggestions.len();
                CompletionsAction::Navigate
            }
            winit::keyboard::KeyCode::ArrowUp => {
                self.ui.selected_index = if self.ui.selected_index == 0 {
                    self.ui.suggestions.len() - 1
                } else {
                    self.ui.selected_index - 1
                };
                CompletionsAction::Navigate
            }
            winit::keyboard::KeyCode::Enter => {
                if let Some(suggestion) = self.ui.suggestions.get(self.ui.selected_index) {
                    CompletionsAction::Accept(suggestion.replacement.clone())
                } else {
                    CompletionsAction::None
                }
            }
            winit::keyboard::KeyCode::Tab => {
                if let Some(suggestion) = self.ui.suggestions.get(self.ui.selected_index) {
                    CompletionsAction::Accept(suggestion.replacement.clone())
                } else {
                    CompletionsAction::None
                }
            }
            winit::keyboard::KeyCode::Escape => {
                self.ui.is_visible = false;
                CompletionsAction::Close
            }
            _ => CompletionsAction::None,
        }
    }

    pub fn add_to_history(&self, command: String) {
        let completion_manager = self.completion_manager.clone();
        tokio::spawn(async move {
            completion_manager.lock().await.add_to_history(&command);
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