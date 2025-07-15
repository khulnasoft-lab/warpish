//! External Tool and Service Integrations
//!
//! This module provides a framework for integrating with external tools
//! and services, such as language servers, debuggers, and other developer tools.

use std::process::{Command, Stdio};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IntegrationError {
    #[error("Command not found: {0}")]
    NotFound(String),
    #[error("Failed to execute command: {0}")]
    ExecutionFailed(std::io::Error),
    #[error("Command returned non-zero exit code: {0}")]
    NonZeroExit(i32),
}

pub struct Integration {
    command: String,
}

impl Integration {
    pub fn new(command: &str) -> Self {
        Self { command: command.to_string() }
    }

    pub fn execute(&self, args: &[&str]) -> Result<String, IntegrationError> {
        let output = Command::new(&self.command)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    IntegrationError::NotFound(self.command.clone())
                } else {
                    IntegrationError::ExecutionFailed(e)
                }
            })?;

        if !output.status.success() {
            return Err(IntegrationError::NonZeroExit(output.status.code().unwrap_or(1)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration_execution() {
        // Test with a simple command that should exist on most systems.
        let integration = Integration::new("echo");
        let result = integration.execute(&["hello", "world"]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "hello world");
    }

    #[test]
    fn test_integration_command_not_found() {
        let integration = Integration::new("this_command_should_not_exist");
        let result = integration.execute(&[]);
        assert!(matches!(result, Err(IntegrationError::NotFound(_))));
    }
}
