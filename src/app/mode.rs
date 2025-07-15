// STUB: AppMode enum and related state structs will go here.
use crate::workflow_manager::Workflow;
use crate::agent::client::FileDiff;

#[derive(Debug, Clone, PartialEq)]
pub struct HistorySearchState {
    pub query: String,
    pub filtered_list: Vec<String>,
    pub selected_idx: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct CommandPaletteState {
    // fields
}
#[derive(Debug, Clone, PartialEq)]
pub struct WorkflowState {
    pub query: String,
    pub filtered_workflows: Vec<Workflow>,
    pub selected_workflow_idx: usize,
    pub execution_state: Option<WorkflowExecutionState>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct WorkflowExecutionState {
    pub workflow: Workflow,
    pub argument_values: Vec<String>,
    pub selected_arg_idx: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AgentState {
    // fields
}
#[derive(Debug, Clone, PartialEq)]
pub struct DiffReviewState {
    pub explanation: String,
    pub files: Vec<FileDiff>,
    pub current_file_idx: usize,
    pub current_hunk_idx: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DriveState {
    pub flat_items: Vec<(String, usize)>,
    pub selected_idx: usize,
}


#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Normal,
    Input,
    HistorySearch(HistorySearchState),
    CommandPalette(CommandPaletteState),
    Workflow(WorkflowState),
    Agent(AgentState),
    AgentManagement,
    DiffReview(DiffReviewState),
    Drive(DriveState),
} 