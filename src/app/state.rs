use crate::{
    agent::Agent,
    app::{
        config::Config,
        drive::DriveManager,
        mode::AppMode,
        pane_manager::{BlockManager, PaneManager},
    },
    config::theme::Theme,
    keybindings::Keybindings,
    mcq::MCQManager,
    natural_language_detection::NaturalLanguageDetector,
    workflow_manager::WorkflowManager,
};
use arboard::Clipboard;
use cosmic_text::{Buffer, Edit, Editor, FontSystem};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum VimAction {
    MoveLeft,
    MoveRight,
    MoveUp,
    MoveDown,
    Paste,
    Enter,
}

#[derive(Debug, Clone)]
pub struct VimState {
    pub mode: VimMode,
    pub pending_action: Option<VimAction>,
}

impl Default for VimState {
    fn default() -> Self {
        Self {
            mode: VimMode::Normal,
            pending_action: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellCommand {
    pub command: String,
    pub args: Vec<String>,
    pub shell: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShellType {
    Bash,
    Fish,
    Zsh,
}

pub struct App {
    pub should_quit: bool,
    pub panes: PaneManager,
    pub active_pane_idx: usize,
    pub config: Config,
    pub keybindings: Keybindings,
    pub mode: AppMode,
    pub workflow_manager: WorkflowManager,
    pub drive_manager: DriveManager,
    pub autosuggestion: Option<String>,
    pub db_conn: Arc<Mutex<rusqlite::Connection>>,
    pub vim_state: Option<VimState>,
    pub input_editor: Editor<'static>,
    pub nld: NaturalLanguageDetector,
    pub agent: Agent,
    pub mcq_manager: MCQManager,
    pub block_manager: Arc<Mutex<BlockManager>>,
    pub theme: Theme, // Add theme to App struct
}

impl App {
    pub fn new(
        config: Config,
        db_conn: Arc<Mutex<rusqlite::Connection>>,
        mut font_system: &mut FontSystem,
        theme: Theme,
    ) -> Self {
        let panes = PaneManager::new(&config);
        let keybindings = crate::keybindings::load_keybindings(&config.keybindings_path).unwrap_or_default();
        let workflow_manager = WorkflowManager::new();
        let drive_path = config
            .drive_path
            .clone()
            .map(PathBuf::from)
            .unwrap_or_else(|| dirs::home_dir().unwrap().join(".warpish-drive"));
        let drive_manager = DriveManager::load_from_disk(&drive_path);
        let vim_state = if config.vim_mode {
            Some(VimState::default())
        } else {
            None
        };

        let buffer = Buffer::new(&mut font_system, cosmic_text::Metrics::new(14.0, 20.0));
        let input_editor = Editor::new(buffer);
        let nld = NaturalLanguageDetector::new();
        let agent = Agent::new(config.openai_api_key.clone());
        let mcq_manager = MCQManager::new();
        let block_manager = Arc::new(Mutex::new(BlockManager::default()));

        Self {
            should_quit: false,
            panes,
            active_pane_idx: 0,
            config,
            keybindings,
            mode: AppMode::Normal,
            workflow_manager,
            drive_manager,
            autosuggestion: None,
            db_conn,
            vim_state,
            input_editor,
            nld,
            agent,
            mcq_manager,
            block_manager,
            theme,
        }
    }

    pub fn on_tick(&mut self) {
        // Here you could update animations, check for agent task completion, etc.
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, clipboard: &mut Clipboard) -> anyhow::Result<()> {
        if let Some(vim_state) = &mut self.vim_state {
            match vim_state.mode {
                VimMode::Normal => self.handle_vim_normal_mode(key_event, clipboard)?,
                VimMode::Insert => self.handle_insert_mode(key_event, clipboard)?,
                VimMode::Visual => { /* Handle visual mode keys */ }
            }
        } else {
            self.handle_insert_mode(key_event, clipboard)?;
        }
        Ok(())
    }

    fn handle_vim_normal_mode(&mut self, key_event: KeyEvent, clipboard: &mut Clipboard) -> anyhow::Result<()> {
        let action = match key_event.code {
            KeyCode::Char('h') => Some(VimAction::MoveLeft),
            KeyCode::Char('j') => Some(VimAction::MoveDown),
            KeyCode::Char('k') => Some(VimAction::MoveUp),
            KeyCode::Char('l') => Some(VimAction::MoveRight),
            KeyCode::Char('p') => Some(VimAction::Paste),
            KeyCode::Char('i') => {
                if let Some(vim) = &mut self.vim_state {
                    vim.mode = VimMode::Insert;
                }
                None
            }
            KeyCode::Enter => Some(VimAction::Enter),
            KeyCode::Esc => {
                self.mode = AppMode::Normal;
                None
            }
            _ => None,
        };

        if let Some(action) = action {
            self.perform_vim_action(action, clipboard);
        }

        Ok(())
    }

    fn perform_vim_action(&mut self, action: VimAction, clipboard: &mut Clipboard) {
        match action {
            VimAction::Paste => {
                if let Ok(text) = clipboard.get_text() {
                    self.input_editor.insert_string(&text, None);
                }
            }
            VimAction::Enter => {
                // Get text from editor (simplified)
                let command = "placeholder_command".to_string();
                self.panes.get_active_pane().unwrap().write_to_pty(&command);
                // Clear editor (simplified)
                // self.input_editor.buffer_ref_mut().set_text(...);
            }
            _ => {}
        }
    }

    fn handle_insert_mode(
        &mut self,
        key_event: KeyEvent,
        _clipboard: &mut Clipboard,
    ) -> anyhow::Result<()> {
        match key_event.code {
            KeyCode::Char(c) => {
                if !key_event.modifiers.contains(KeyModifiers::CONTROL) {
                    // Simplified text insertion
                    self.input_editor.insert_string(&c.to_string(), None);
                }
            }
            KeyCode::Backspace => {
                // Simplified backspace handling
                // self.input_editor.delete_selection();
            }
            KeyCode::Enter => {
                // Simplified command execution
                let command = "placeholder_command".to_string();
                self.panes.get_active_pane().unwrap().write_to_pty(&command);
                // Clear editor (simplified)
            }
            KeyCode::Esc => {
                 if let Some(vim) = &mut self.vim_state {
                    vim.mode = VimMode::Normal;
                }
            }
            _ => {}
        }
        Ok(())
    }
}