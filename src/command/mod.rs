//! Command Module
//! 
//! This module handles command parsing, execution, and management
//! for the terminal, including built-in commands and external process execution.

use std::collections::HashMap;
use std::process::{Command as StdCommand, Stdio};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Command not found: {0}")]
    NotFound(String),
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    #[error("Parse error: {0}")]
    ParseError(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandInfo {
    pub name: String,
    pub description: String,
    pub usage: String,
    pub examples: Vec<String>,
}

pub struct CommandParser {
    builtin_commands: HashMap<String, CommandInfo>,
}

impl CommandParser {
    pub fn new() -> Self {
        let mut builtin_commands = HashMap::new();
        
        // Add built-in commands
        builtin_commands.insert("help".to_string(), CommandInfo {
            name: "help".to_string(),
            description: "Show help information".to_string(),
            usage: "help [command]".to_string(),
            examples: vec!["help".to_string(), "help ls".to_string()],
        });
        
        builtin_commands.insert("clear".to_string(), CommandInfo {
            name: "clear".to_string(),
            description: "Clear the terminal screen".to_string(),
            usage: "clear".to_string(),
            examples: vec!["clear".to_string()],
        });
        
        Self { builtin_commands }
    }
    
    pub fn parse(&self, input: &str) -> Result<Vec<String>, CommandError> {
        shellwords::split(input)
            .map_err(|e| CommandError::ParseError(e.to_string()))
    }
    
    pub fn is_builtin(&self, command: &str) -> bool {
        self.builtin_commands.contains_key(command)
    }
    
    pub fn get_builtin_info(&self, command: &str) -> Option<&CommandInfo> {
        self.builtin_commands.get(command)
    }
    
    pub fn execute_builtin(&self, command: &str, args: &[String]) -> Result<String, CommandError> {
        match command {
            "help" => self.handle_help(args),
            "clear" => Ok("\x1b[2J\x1b[H".to_string()),
            _ => Err(CommandError::NotFound(command.to_string())),
        }
    }
    
    fn handle_help(&self, args: &[String]) -> Result<String, CommandError> {
        if args.is_empty() {
            let mut help_text = String::from("Available commands:\n");
            for (name, info) in &self.builtin_commands {
                help_text.push_str(&format!("  {}: {}\n", name, info.description));
            }
            Ok(help_text)
        } else {
            let command = &args[0];
            if let Some(info) = self.builtin_commands.get(command) {
                Ok(format!("{}\n\nUsage: {}\n\nExamples:\n{}", 
                    info.description, 
                    info.usage,
                    info.examples.join("\n")))
            } else {
                Err(CommandError::NotFound(command.to_string()))
            }
        }
    }
}

pub async fn execute_external_command(command: &str, args: &[String]) -> Result<String, CommandError> {
    let output = StdCommand::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| CommandError::ExecutionFailed(e.to_string()))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    if output.status.success() {
        Ok(stdout.to_string())
    } else {
        Err(CommandError::ExecutionFailed(stderr.to_string()))
    }
}

#[cfg(test)]
mod tests;
