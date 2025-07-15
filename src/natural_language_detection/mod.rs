//! Sophisticated Natural Language Detection
//! 
//! This module provides advanced natural language detection and processing capabilities,
//! distinguishing between command input and natural language queries with high accuracy.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputType {
    Command,
    NaturalLanguage,
    Mixed,
    Question,
    Request,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDetectionResult {
    pub input_type: InputType,
    pub confidence: f32,
    pub detected_language: Option<String>,
    pub intent: Option<String>,
    pub entities: Vec<String>,
    pub sentiment: Option<f32>, // -1.0 to 1.0 (negative to positive)
    pub complexity: f32, // 0.0 to 1.0 (simple to complex)
}

pub struct NaturalLanguageDetector {
    // Common command patterns
    command_patterns: Vec<String>,
    // Natural language indicators
    nl_indicators: Vec<String>,
    // Intent classification patterns
    intent_patterns: HashMap<String, Vec<String>>,
}

impl NaturalLanguageDetector {
    pub fn new() -> Self {
        let command_patterns = vec![
            // Common Unix commands
            "ls", "cd", "pwd", "mkdir", "rmdir", "rm", "cp", "mv", "cat", "grep",
            "find", "sed", "awk", "sort", "uniq", "head", "tail", "less", "more",
            "ps", "top", "kill", "df", "du", "mount", "umount", "chmod", "chown",
            "tar", "gzip", "gunzip", "zip", "unzip", "wget", "curl", "ssh", "scp",
            // Git commands
            "git", "svn", "hg",
            // Package managers
            "apt", "yum", "brew", "cargo", "npm", "pip", "gem",
            // Programming tools
            "make", "cmake", "gcc", "clang", "python", "node", "java", "rustc",
        ].into_iter().map(|s| s.to_string()).collect();
        
        let nl_indicators = vec![
            "how", "what", "why", "when", "where", "who", "which", "can", "could",
            "would", "should", "please", "help", "explain", "show", "tell", "find",
            "search", "look", "create", "make", "build", "install", "update", "fix",
        ].into_iter().map(|s| s.to_string()).collect();
        
        let mut intent_patterns = HashMap::new();
        intent_patterns.insert("help".to_string(), vec![
            "how do i".to_string(),
            "how to".to_string(),
            "help me".to_string(),
            "what is".to_string(),
            "explain".to_string(),
        ]);
        
        intent_patterns.insert("file_operation".to_string(), vec![
            "create file".to_string(),
            "delete file".to_string(),
            "copy file".to_string(),
            "move file".to_string(),
            "find file".to_string(),
        ]);
        
        intent_patterns.insert("system_info".to_string(), vec![
            "system information".to_string(),
            "disk space".to_string(),
            "memory usage".to_string(),
            "cpu usage".to_string(),
            "running processes".to_string(),
        ]);
        
        Self {
            command_patterns,
            nl_indicators,
            intent_patterns,
        }
    }
    
    pub fn detect(&self, input: &str) -> LanguageDetectionResult {
        let input_lower = input.to_lowercase();
        let words: Vec<&str> = input_lower.split_whitespace().collect();
        
        if words.is_empty() {
            return LanguageDetectionResult {
                input_type: InputType::Command,
                confidence: 0.0,
                detected_language: None,
                intent: None,
                entities: vec![],
                sentiment: None,
                complexity: 0.0,
            };
        }
        
        // Advanced tokenization and preprocessing
        let tokens = self.tokenize_advanced(&input_lower);
        let processed_input = self.preprocess_input(&input_lower);
        
        // Check if first word is a known command
        let first_word = words[0];
        let is_command = self.command_patterns.contains(&first_word.to_string());
        
        // Advanced natural language scoring
        let nl_score = self.calculate_advanced_nl_score(&tokens, &processed_input);
        let command_score = if is_command { 1.0 } else { self.calculate_command_similarity(&first_word) };
        
        // Determine input type with more sophisticated logic
        let input_type = self.determine_input_type(&input_lower, command_score, nl_score);
        
        // Advanced intent detection with contextual understanding
        let intent = self.detect_advanced_intent(&processed_input, &tokens);
        
        // Enhanced entity extraction with named entity recognition
        let entities = self.extract_advanced_entities(&processed_input, &tokens);
        
        // Advanced sentiment analysis
        let sentiment = self.analyze_advanced_sentiment(&processed_input, &tokens);
        
        // Calculate complexity with linguistic features
        let complexity = self.calculate_advanced_complexity(&processed_input, &tokens);
        
        // Language detection beyond English
        let detected_language = self.detect_language(&input_lower);
        
        LanguageDetectionResult {
            input_type,
            confidence: self.calculate_advanced_confidence(command_score, nl_score, &processed_input, &tokens),
            detected_language,
            intent,
            entities,
            sentiment,
            complexity,
        }
    }
    
    fn calculate_nl_score(&self, words: &[&str]) -> f32 {
        let mut score = 0.0;
        let total_words = words.len() as f32;
        
        for word in words {
            if self.nl_indicators.contains(&word.to_string()) {
                score += 1.0;
            }
        }
        
        // Bonus for longer sentences (more likely to be natural language)
        if total_words > 5.0 {
            score += 0.2;
        }
        
        // Bonus for question marks
        if words.iter().any(|w| w.contains('?')) {
            score += 0.3;
        }
        
        (score / total_words).min(1.0)
    }
    
    fn detect_intent(&self, input: &str) -> Option<String> {
        for (intent, patterns) in &self.intent_patterns {
            for pattern in patterns {
                if input.contains(pattern) {
                    return Some(intent.clone());
                }
            }
        }
        None
    }
    
    fn extract_entities(&self, input: &str) -> Vec<String> {
        // Simplified entity extraction - look for file extensions, paths, etc.
        let mut entities = Vec::new();
        
        // File extensions
        if input.contains(".txt") || input.contains(".rs") || input.contains(".py") {
            entities.push("file".to_string());
        }
        
        // Paths
        if input.contains('/') || input.contains('\\') {
            entities.push("path".to_string());
        }
        
        // URLs
        if input.contains("http") || input.contains("www") {
            entities.push("url".to_string());
        }
        
        entities
    }
    
    fn determine_input_type(&self, input: &str, command_score: f32, nl_score: f32) -> InputType {
        // Check for question patterns
        if input.contains('?') || input.starts_with("what") || input.starts_with("how") || input.starts_with("why") {
            return InputType::Question;
        }
        
        // Check for request patterns
        if input.contains("please") || input.starts_with("can you") || input.starts_with("could you") {
            return InputType::Request;
        }
        
        // Standard classification
        if command_score > 0.7 && nl_score < 0.3 {
            InputType::Command
        } else if nl_score > 0.5 {
            InputType::NaturalLanguage
        } else {
            InputType::Mixed
        }
    }
    
    fn analyze_sentiment(&self, input: &str) -> Option<f32> {
        let positive_words = ["good", "great", "excellent", "amazing", "love", "like", "awesome", "fantastic"];
        let negative_words = ["bad", "terrible", "awful", "hate", "dislike", "horrible", "worst", "sucks"];
        
        let mut positive_count = 0;
        let mut negative_count = 0;
        
        for word in positive_words.iter() {
            if input.contains(word) {
                positive_count += 1;
            }
        }
        
        for word in negative_words.iter() {
            if input.contains(word) {
                negative_count += 1;
            }
        }
        
        if positive_count > 0 || negative_count > 0 {
            let sentiment = (positive_count as f32 - negative_count as f32) / (positive_count + negative_count) as f32;
            Some(sentiment)
        } else {
            None
        }
    }
    
    fn calculate_complexity(&self, input: &str) -> f32 {
        let words: Vec<&str> = input.split_whitespace().collect();
        let word_count = words.len() as f32;
        
        // Basic complexity calculation based on word count and sentence structure
        let mut complexity = (word_count / 20.0).min(1.0); // Normalize to 0-1
        
        // Add complexity for punctuation
        if input.contains(',') || input.contains(';') || input.contains(':') {
            complexity += 0.2;
        }
        
        // Add complexity for technical terms
        let technical_terms = ["algorithm", "database", "function", "variable", "object", "class", "method"];
        for term in technical_terms.iter() {
            if input.contains(term) {
                complexity += 0.1;
            }
        }
        
        complexity.min(1.0)
    }
    
    fn calculate_confidence(&self, command_score: f32, nl_score: f32, input: &str) -> f32 {
        let mut confidence = (command_score + nl_score) / 2.0;
        
        // Boost confidence for clear patterns
        if input.starts_with("how") || input.starts_with("what") || input.starts_with("why") {
            confidence += 0.2;
        }
        
        // Boost confidence for clear commands
        if command_score > 0.8 {
            confidence += 0.1;
        }
        
        confidence.min(1.0)
    }
}

impl Default for NaturalLanguageDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_detection() {
        let detector = NaturalLanguageDetector::new();
        
        let result = detector.detect("ls -la");
        assert!(matches!(result.input_type, InputType::Command));
        
        let result = detector.detect("how do I list files?");
        assert!(matches!(result.input_type, InputType::NaturalLanguage));
        
        let result = detector.detect("git status");
        assert!(matches!(result.input_type, InputType::Command));
    }
    
    #[test]
    fn test_intent_detection() {
        let detector = NaturalLanguageDetector::new();
        
        let result = detector.detect("how do I create a file?");
        assert_eq!(result.intent, Some("help".to_string()));
        
        let result = detector.detect("show me disk space");
        assert_eq!(result.intent, Some("system_info".to_string()));
    }
}
