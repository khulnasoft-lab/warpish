use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use thiserror::Error;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Error, Debug)]
pub enum RuleError {
    #[error("File I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parsing error: {0}")]
    Scan(#[from] yaml_rust::ScanError),
    #[error("Rule format is invalid: {0}")]
    InvalidFormat(String),
}

/// Defines the possible actions a rule can take.
#[derive(Debug, Serialize, Deserialize)]
pub enum RuleAction {
    RunCommand(String),
    SuggestFix(String),
    OpenUrl(String),
}

/// Represents a single AI rule that can be triggered.
#[derive(Debug, Serialize, Deserialize)]
pub struct Rule {
    pub name: String,
    pub trigger_phrase: String,
    pub action: RuleAction,
}

/// Loads a vector of Rules from a given YAML file path.
pub fn load_rules_from_yaml(path: &Path) -> Result<Vec<Rule>, RuleError> {
    let content = fs::read_to_string(path)?;
    let docs = YamlLoader::load_from_str(&content)?;
    let doc = docs.get(0).ok_or_else(|| RuleError::InvalidFormat("YAML file is empty".to_string()))?;
    let mut rules = Vec::new();
    if let Yaml::Array(rule_docs) = doc {
        for rule_doc in rule_docs {
            let name = rule_doc["name"].as_str().ok_or_else(|| RuleError::InvalidFormat("Missing 'name' field".to_string()))?.to_string();
            let trigger_phrase = rule_doc["trigger_phrase"].as_str().ok_or_else(|| RuleError::InvalidFormat("Missing 'trigger_phrase' field".to_string()))?.to_string();
            let action_doc = &rule_doc["action"];
            let action = if let Some(cmd) = action_doc["RunCommand"].as_str() {
                RuleAction::RunCommand(cmd.to_string())
            } else if let Some(sugg) = action_doc["SuggestFix"].as_str() {
                RuleAction::SuggestFix(sugg.to_string())
            } else if let Some(url) = action_doc["OpenUrl"].as_str() {
                RuleAction::OpenUrl(url.to_string())
            } else {
                return Err(RuleError::InvalidFormat(format!("Invalid or missing action for rule '{}'", name)));
            };
            rules.push(Rule { name, trigger_phrase, action });
        }
    } else {
        return Err(RuleError::InvalidFormat("Expected top-level YAML element to be an array".to_string()));
    }
    Ok(rules)
} 