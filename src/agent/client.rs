use crate::agent::model::ModelId; // Import the new enum
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use serde::{Deserialize, Serialize};
use similar::{ChangeTag, TextDiff};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FileDiff {
    pub file_path: String,
    pub original_content: String,
    pub new_content: String,
    pub hunks: Vec<Hunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiffPatch {
    pub explanation: String,
    pub diffs: Vec<FileDiff>,
}

// FIX: ChangeTag now correctly derives Serialize and Deserialize
// because the `serde` feature is enabled for the `similar` crate.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Hunk {
    pub tag: ChangeTag,
    pub original_text: String,
    pub new_text: String,
}

impl FileDiff {
    pub fn from_changes(file_path: &str, old: &str, new: &str) -> Self {
        let diff = TextDiff::from_lines(old, new);
        let hunks = diff
            .iter_all_changes()
            .map(|change| Hunk {
                tag: change.tag(),
                // FIX: Updated to use the new `value()` API from `similar`
                original_text: if change.tag() != ChangeTag::Insert {
                    change.value().to_string()
                } else {
                    String::new()
                },
                new_text: if change.tag() != ChangeTag::Delete {
                    change.value().to_string()
                } else {
                    String::new()
                },
            })
            .collect();

        Self {
            file_path: file_path.to_string(),
            original_content: old.to_string(),
            new_content: new.to_string(),
            hunks,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentResponse {
    SuggestCommand {
        explanation: String,
        command: String,
    },
    RequestToRunCommand {
        explanation: String,
        command_to_run: String,
    },
    Clarification(String),
    ProposeCodeChange {
        diffs: Vec<FileDiff>,
        explanation: String,
    },
}

pub struct SimulatedAgent {
    matcher: SkimMatcherV2,
}

impl SimulatedAgent {
    pub fn new() -> Self {
        Self { matcher: SkimMatcherV2::default() }
    }

    // Update the signature to accept the model
    pub fn process_query(
        &self,
        query: &str,
        history: &[(String, AgentResponse)],
        block_context: &[String],
        model: ModelId, // Add model parameter
    ) -> AgentResponse {
        log::info!("Simulating agent processing with model: {}", model.to_string());
        // --- Simulation for Code Change ---
        if query.starts_with("refactor") {
            if let Some(file_path) = query.split_whitespace().nth(1) {
                match std::fs::read_to_string(file_path) {
                    Ok(original_content) => {
                        let new_content = format!("// This file has been refactored by Warpish AI.\n\n{}", original_content);
                        let hunks = FileDiff::from_changes(file_path, &original_content, &new_content).hunks;
                        return AgentResponse::ProposeCodeChange {
                            diffs: vec![FileDiff { 
                                file_path: file_path.to_string(), 
                                original_content: original_content.clone(),
                                new_content,
                                hunks,
                            }],
                            explanation: format!("I've added a comment to the top of `{}`. Review the changes and apply if correct.", file_path),
                        };
                    }
                    Err(e) => {
                        return AgentResponse::Clarification(format!("Failed to read '{}': {}", file_path, e));
                    }
                }
            }
        }
        // Our simulation's behavior doesn't change, but in a real app,
        // you would use this parameter to make an API call to the correct endpoint.

        // A real LLM would use this context. Our simulation can check for keywords.
        if block_context.iter().any(|b| b.contains("error") || b.contains("failed")) {
            if query.contains("fix this") {
                return AgentResponse::SuggestCommand {
                    explanation: "It looks like there was a build error. I can try to clean the project and rebuild.".into(),
                    command: "cargo clean && cargo build".into(),
                };
            }
        }
        // In a real LLM, you would use the history. For our simulation,
        // we'll just continue to match against the latest query.
        let known_tasks = [
            ("list all docker containers", AgentResponse::SuggestCommand {
                explanation: "To see all Docker containers, including stopped ones, you should use `docker ps -a`.".into(),
                command: "docker ps -a".into(),
            }),
            ("how do i see my git branches", AgentResponse::SuggestCommand {
                explanation: "You can list all local and remote git branches with `git branch -a`.".into(),
                command: "git branch -a".into(),
            }),
            ("what is my ip address", AgentResponse::RequestToRunCommand {
                explanation: "To determine your public IP address, I need to run a command to fetch it. Is it okay to run `curl ifconfig.me`?".into(),
                command_to_run: "curl ifconfig.me".into(),
            }),
        ];
        if let Some((_, response)) = known_tasks.iter()
            .max_by_key(|(task, _)| self.matcher.fuzzy_match(task, query).unwrap_or(0))
        {
            return response.clone();
        }
        AgentResponse::Clarification("I'm not sure how to help with that. Could you be more specific?".into())
    }
} 