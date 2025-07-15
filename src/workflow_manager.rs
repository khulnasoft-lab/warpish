// STUB: WorkflowManager and Workflow structs will go here.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Workflow {
    pub name: String,
    pub description: String,
    pub command: String,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Argument {
    pub name: String,
    pub description: String,
}

pub struct WorkflowManager;

impl WorkflowManager {
    pub fn new() -> Self { Self }
} 