use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
    Visual,
}

#[derive(Debug, Clone, Copy)]
pub enum VimAction {
    NoOp,
    EnterInsertMode,
    EnterInsertModeAtEnd,
    EnterNormalMode,
    EnterVisualMode,
    Move(VimMotion),
    Delete(VimMotion),
    Yank(VimMotion), // Copy
    Paste,
    Undo,
    Redo,
}

#[derive(Debug, Clone, Copy)]
pub enum VimMotion {
    Left, Right, Up, Down,
    WordForward, WordBackward,
    LineStart, LineEnd,
}

#[derive(Debug, Clone)]
pub struct VimState {
    pub mode: VimMode,
    pending_operator: Option<char>,
}

impl Default for VimState {
    fn default() -> Self {
        // Vim starts in Insert mode in Warpish, as per the spec.
        Self { mode: VimMode::Insert, pending_operator: None }
    }
}

impl VimState {
    /// This is the core Vim command parser.
    pub fn handle_key(&mut self, key: &winit::event::KeyEvent) -> VimAction {
        if self.mode == VimMode::Insert {
            // In Insert mode, only Escape does something special.
            return if key.physical_key == PhysicalKey::Code(KeyCode::Escape) {
                VimAction::EnterNormalMode
            } else {
                VimAction::NoOp // Other keys are handled as text input directly
            };
        }

        // --- Normal and Visual Mode Logic ---
        let key_char = match key.physical_key {
            PhysicalKey::Code(KeyCode::KeyA) => Some('a'),
            PhysicalKey::Code(KeyCode::KeyB) => Some('b'),
            PhysicalKey::Code(KeyCode::KeyD) => Some('d'),
            PhysicalKey::Code(KeyCode::KeyE) => Some('e'),
            PhysicalKey::Code(KeyCode::KeyG) => Some('g'),
            PhysicalKey::Code(KeyCode::KeyH) => Some('h'),
            PhysicalKey::Code(KeyCode::KeyI) => Some('i'),
            PhysicalKey::Code(KeyCode::KeyJ) => Some('j'),
            PhysicalKey::Code(KeyCode::KeyK) => Some('k'),
            PhysicalKey::Code(KeyCode::KeyL) => Some('l'),
            PhysicalKey::Code(KeyCode::KeyP) => Some('p'),
            PhysicalKey::Code(KeyCode::KeyU) => Some('u'),
            PhysicalKey::Code(KeyCode::KeyV) => Some('v'),
            PhysicalKey::Code(KeyCode::KeyW) => Some('w'),
            PhysicalKey::Code(KeyCode::KeyY) => Some('y'),
            PhysicalKey::Code(KeyCode::Escape) => Some('\x1B'),
            _ => None,
        };
        
        if let Some(op) = self.pending_operator {
            self.pending_operator = None;
            let motion = match key_char {
                Some('w') => VimMotion::WordForward,
                Some('b') => VimMotion::WordBackward,
                // `dw` is delete word, `dd` would be delete line, etc.
                Some('d') if op == 'd' => VimMotion::LineEnd, // Simplified: dd = delete line
                _ => return VimAction::NoOp,
            };
            return match op {
                'd' => VimAction::Delete(motion),
                'y' => VimAction::Yank(motion),
                _ => VimAction::NoOp,
            };
        }

        match key_char {
            Some('i') => VimAction::EnterInsertMode,
            Some('a') => VimAction::EnterInsertModeAtEnd,
            Some('v') => VimAction::EnterVisualMode,
            Some('p') => VimAction::Paste,
            Some('u') => VimAction::Undo,
            Some('d') | Some('y') => { self.pending_operator = key_char; VimAction::NoOp },
            Some('h') => VimAction::Move(VimMotion::Left),
            Some('l') => VimAction::Move(VimMotion::Right),
            Some('k') => VimAction::Move(VimMotion::Up),
            Some('j') => VimAction::Move(VimMotion::Down),
            Some('w') => VimAction::Move(VimMotion::WordForward),
            Some('b') => VimAction::Move(VimMotion::WordBackward),
            Some('\x1B') => { self.mode = VimMode::Normal; VimAction::NoOp },
            _ => VimAction::NoOp,
        }
    }
} 