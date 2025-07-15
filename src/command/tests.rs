//! Comprehensive tests for the command module
//! 
//! This module contains integration tests and benchmarks for command processing.

use super::*;
use std::time::Instant;

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[test]
    fn test_command_parser_creation() {
        let parser = CommandParser::new();
        assert!(parser.is_builtin("help"));
        assert!(parser.is_builtin("clear"));
        assert!(!parser.is_builtin("nonexistent"));
    }

    #[test]
    fn test_command_parsing() {
        let parser = CommandParser::new();
        
        // Test simple command
        let result = parser.parse("ls").unwrap();
        assert_eq!(result, vec!["ls"]);
        
        // Test command with arguments
        let result = parser.parse("ls -la /tmp").unwrap();
        assert_eq!(result, vec!["ls", "-la", "/tmp"]);
        
        // Test quoted arguments
        let result = parser.parse("echo 'hello world'").unwrap();
        assert_eq!(result, vec!["echo", "hello world"]);
        
        // Test complex command with pipes (should still parse as separate arguments)
        let result = parser.parse("grep 'pattern' file.txt | sort").unwrap();
        assert!(result.contains(&"grep".to_string()));
        assert!(result.contains(&"pattern".to_string()));
    }

    #[test]
    fn test_builtin_command_execution() {
        let parser = CommandParser::new();
        
        // Test help command
        let result = parser.execute_builtin("help", &[]).unwrap();
        assert!(result.contains("Available commands:"));
        assert!(result.contains("help"));
        assert!(result.contains("clear"));
        
        // Test help with specific command
        let result = parser.execute_builtin("help", &["help".to_string()]).unwrap();
        assert!(result.contains("Show help information"));
        assert!(result.contains("Usage: help"));
        
        // Test clear command
        let result = parser.execute_builtin("clear", &[]).unwrap();
        assert!(result.contains("\x1b[2J\x1b[H"));
        
        // Test nonexistent builtin
        let result = parser.execute_builtin("nonexistent", &[]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::NotFound(_)));
    }

    #[test]
    fn test_command_info_retrieval() {
        let parser = CommandParser::new();
        
        let help_info = parser.get_builtin_info("help").unwrap();
        assert_eq!(help_info.name, "help");
        assert_eq!(help_info.description, "Show help information");
        assert_eq!(help_info.usage, "help [command]");
        assert!(!help_info.examples.is_empty());
        
        let nonexistent_info = parser.get_builtin_info("nonexistent");
        assert!(nonexistent_info.is_none());
    }

    #[test]
    fn test_error_handling() {
        let parser = CommandParser::new();
        
        // Test parsing error with unclosed quote
        let result = parser.parse("echo 'unclosed quote");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::ParseError(_)));
        
        // Test execution error with unknown builtin
        let result = parser.execute_builtin("unknown", &[]);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::NotFound(_)));
    }

    #[test]
    fn test_command_validation() {
        let parser = CommandParser::new();
        
        // Test empty command
        let result = parser.parse("");
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert!(parsed.is_empty());
        
        // Test whitespace-only command
        let result = parser.parse("   ").unwrap();
        assert!(result.is_empty());
        
        // Test command with multiple spaces
        let result = parser.parse("ls    -la     /tmp").unwrap();
        assert_eq!(result, vec!["ls", "-la", "/tmp"]);
    }

    #[tokio::test]
    async fn test_external_command_execution() {
        // Test successful command
        let result = execute_external_command("echo", &["hello".to_string()]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "hello");
        
        // Test command with arguments
        let result = execute_external_command("echo", &["hello".to_string(), "world".to_string()]).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().trim(), "hello world");
        
        // Test nonexistent command
        let result = execute_external_command("nonexistent_command_xyz", &[]).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), CommandError::ExecutionFailed(_)));
    }

    #[test]
    fn test_command_history() {
        // This would be part of a command history feature
        let mut history = Vec::new();
        history.push("ls -la".to_string());
        history.push("cd /tmp".to_string());
        history.push("pwd".to_string());
        
        assert_eq!(history.len(), 3);
        assert_eq!(history[0], "ls -la");
        assert_eq!(history.last(), Some(&"pwd".to_string()));
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_command_pipeline() {
        let parser = CommandParser::new();
        
        // Simulate a command pipeline
        let commands = vec![
            "ls -la",
            "grep 'test'",
            "sort",
            "head -10"
        ];
        
        for cmd in commands {
            let parsed = parser.parse(cmd).unwrap();
            assert!(!parsed.is_empty());
            assert!(!parsed[0].is_empty());
        }
    }

    #[test]
    fn test_command_completion() {
        let parser = CommandParser::new();
        
        // Test partial command completion
        let partial = "he";
        let completions: Vec<&str> = parser.builtin_commands
            .keys()
            .filter(|cmd| cmd.starts_with(partial))
            .map(|s| s.as_str())
            .collect();
        
        assert!(completions.contains(&"help"));
    }

    #[test]
    fn test_command_suggestion() {
        let parser = CommandParser::new();
        
        // Test command suggestion for typos
        let typo = "halp";
        let suggestions = find_similar_commands(&parser, typo);
        assert!(suggestions.contains(&"help".to_string()));
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn benchmark_command_parsing() {
        let parser = CommandParser::new();
        let test_commands = vec![
            "ls -la /tmp",
            "grep 'pattern' file.txt",
            "find . -name '*.rs' -type f",
            "ps aux | grep rust",
            "docker run --rm -it ubuntu:latest bash",
        ];
        
        let start = Instant::now();
        for _ in 0..1000 {
            for cmd in &test_commands {
                let _ = parser.parse(cmd);
            }
        }
        let duration = start.elapsed();
        
        println!("Command parsing benchmark: {:?} for 5000 operations", duration);
        assert!(duration.as_millis() < 100); // Should be fast
    }

    #[test]
    fn benchmark_builtin_execution() {
        let parser = CommandParser::new();
        
        let start = Instant::now();
        for _ in 0..1000 {
            let _ = parser.execute_builtin("help", &[]);
            let _ = parser.execute_builtin("clear", &[]);
        }
        let duration = start.elapsed();
        
        println!("Builtin execution benchmark: {:?} for 2000 operations", duration);
        assert!(duration.as_millis() < 50); // Should be very fast
    }
}

// Helper functions for testing
fn find_similar_commands(parser: &CommandParser, input: &str) -> Vec<String> {
    let mut suggestions = Vec::new();
    
    for cmd in parser.builtin_commands.keys() {
        if levenshtein_distance(input, cmd) <= 2 {
            suggestions.push(cmd.clone());
        }
    }
    
    suggestions
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let len1 = s1.len();
    let len2 = s2.len();
    let mut matrix = vec![vec![0; len2 + 1]; len1 + 1];
    
    for i in 0..=len1 {
        matrix[i][0] = i;
    }
    for j in 0..=len2 {
        matrix[0][j] = j;
    }
    
    for i in 1..=len1 {
        for j in 1..=len2 {
            let cost = if s1.chars().nth(i - 1) == s2.chars().nth(j - 1) { 0 } else { 1 };
            matrix[i][j] = std::cmp::min(
                std::cmp::min(matrix[i - 1][j] + 1, matrix[i][j - 1] + 1),
                matrix[i - 1][j - 1] + cost,
            );
        }
    }
    
    matrix[len1][len2]
}
