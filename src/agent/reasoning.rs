// Agent reasoning module

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentState {
    pub context: String,
    pub reasoning_chain: Vec<String>,
    pub confidence: f32,
    pub metadata: HashMap<String, String>,
}

impl AgentState {
    pub fn new() -> Self {
        Self {
            context: String::new(),
            reasoning_chain: Vec::new(),
            confidence: 0.0,
            metadata: HashMap::new(),
        }
    }
    
    pub fn with_context(context: String) -> Self {
        Self {
            context,
            reasoning_chain: Vec::new(),
            confidence: 0.0,
            metadata: HashMap::new(),
        }
    }
    
    pub fn add_reasoning_step(&mut self, step: String) {
        self.reasoning_chain.push(step);
    }
    
    pub fn set_confidence(&mut self, confidence: f32) {
        self.confidence = confidence.clamp(0.0, 1.0);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReasoningContext {
    pub current_task: String,
    pub available_tools: Vec<String>,
    pub history: Vec<String>,
}

impl ReasoningContext {
    pub fn new() -> Self {
        Self {
            current_task: String::new(),
            available_tools: Vec::new(),
            history: Vec::new(),
        }
    }
}

pub trait ReasoningEngine {
    fn process(&mut self, context: &ReasoningContext) -> AgentState;
    fn update_state(&mut self, state: AgentState);
}

pub struct SimpleReasoningEngine {
    current_state: AgentState,
}

impl SimpleReasoningEngine {
    pub fn new() -> Self {
        Self {
            current_state: AgentState::new(),
        }
    }
}

impl ReasoningEngine for SimpleReasoningEngine {
    fn process(&mut self, context: &ReasoningContext) -> AgentState {
        // Simple reasoning implementation
        let mut state = AgentState::new();
        state.context = context.current_task.clone();
        state.add_reasoning_step("Analyzing task...".to_string());
        state.set_confidence(0.7);
        state
    }
    
    fn update_state(&mut self, state: AgentState) {
        self.current_state = state;
    }
}
