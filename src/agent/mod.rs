pub mod client;
pub mod model;

// FIX: Make structs and enums public
pub use client::{DiffPatch, FileDiff, Hunk, AgentResponse};
pub use model::ModelId;

use crate::app::state::ShellCommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    RunCommand(ShellCommand),
    ApplyPatch(DiffPatch),
    Allow, // FIX: Added missing variant
}

pub struct Agent {
    // ... fields
}

impl Agent {
    pub fn new(api_key: Option<String>) -> Self {
        Self { /* ... */ }
    }
}

// FIX: Added missing fields to the FileDiff initialization.
pub fn some_function_creating_a_diff(file_path: &str, new_content: String, old_content: String) -> DiffPatch {
    let diffs = vec![FileDiff {
        file_path: file_path.to_string(),
        new_content,
        original_content: old_content.clone(), // FIX: Added missing field
        hunks: FileDiff::from_changes(file_path, &old_content, "").hunks, // FIX: Added missing field
    }];
    DiffPatch {
        explanation: "Some explanation".to_string(),
        diffs,
    }
}