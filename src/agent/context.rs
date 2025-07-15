use std::collections::HashMap;

#[derive(Debug, Default, Clone)]
pub struct AgentContext {
    pub memory: Vec<String>,
    pub variables: HashMap<String, String>,
    pub last_input: Option<String>,
}

impl AgentContext {
    pub fn update_memory(&mut self, entry: String) {
        self.memory.push(entry);
    }

    pub fn set_variable(&mut self, key: &str, value: &str) {
        self.variables.insert(key.to_string(), value.to_string());
    }

    pub fn get_variable(&self, key: &str) -> Option<&String> {
        self.variables.get(key)
    }
}