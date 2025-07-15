use crate::agent::model::ModelId; // Import the new enum
use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub file_path: String,
    pub new_content: String,
}

#[derive(Debug, Clone)]
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
                        return AgentResponse::ProposeCodeChange {
                            diffs: vec![FileDiff { file_path: file_path.to_string(), new_content }],
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