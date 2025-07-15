use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use std::{collections::HashMap, fs, path::Path};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub display: String,
    pub replacement: String,
    pub description: Option<String>,
    pub suggestion_type: SuggestionType,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SuggestionType {
    Command,
    Subcommand,
    Flag,
    Argument,
    FilePath,
    History,
    AiGenerated,
    Workflow,
}

/// A trait for any object that can provide completion suggestions.
trait Completer {
    fn suggest(&self, context: &str) -> Vec<Suggestion>;
}

/// A spec for a specific command, like "git" or "docker".
struct CommandSpec {
    subcommands: Vec<String>,
    flags: Vec<String>,
    description: String,
}

impl Completer for CommandSpec {
    fn suggest(&self, context: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        // Add subcommands
        for subcmd in &self.subcommands {
            if subcmd.starts_with(context) {
                suggestions.push(Suggestion {
                    display: subcmd.clone(),
                    replacement: subcmd.clone(),
                    description: Some(format!("{} subcommand", self.description)),
                    suggestion_type: SuggestionType::Subcommand,
                    confidence: 0.9,
                });
            }
        }
        
        // Add flags
        for flag in &self.flags {
            if flag.starts_with(context) {
                suggestions.push(Suggestion {
                    display: flag.clone(),
                    replacement: flag.clone(),
                    description: Some(format!("{} flag", self.description)),
                    suggestion_type: SuggestionType::Flag,
                    confidence: 0.8,
                });
            }
        }
        
        suggestions
    }
}

/// A completer for file and directory paths.
struct FilePathCompleter;

impl Completer for FilePathCompleter {
    fn suggest(&self, context: &str) -> Vec<Suggestion> {
        let path = Path::new(context);
        let (dir, partial) = if context.ends_with('/') || context.is_empty() {
            (path, "")
        } else {
            (path.parent().unwrap_or_else(|| Path::new(".")), path.file_name().unwrap_or_default().to_str().unwrap())
        };
        
        let mut suggestions = Vec::new();
        if let Ok(entries) = fs::read_dir(dir.to_str().unwrap_or(".")) {
            for entry in entries.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.starts_with(partial) {
                        let mut replacement = dir.join(name).to_string_lossy().to_string();
                        if entry.path().is_dir() {
                            replacement.push('/');
                        }
                        let description = if entry.path().is_dir() {
                            Some("Directory".to_string())
                        } else {
                            Some("File".to_string())
                        };
                        suggestions.push(Suggestion {
                            display: name.to_string(),
                            replacement,
                            description,
                            suggestion_type: SuggestionType::FilePath,
                            confidence: 0.7,
                        });
                    }
                }
            }
        }
        suggestions
    }
}

/// AI-powered completer that uses LLM for intelligent suggestions
pub struct AiCompleter {
    client: Arc<Mutex<Option<reqwest::Client>>>,
    api_url: String,
    model: String,
}

impl AiCompleter {
    pub fn new() -> Self {
        let api_url = std::env::var("OLLAMA_API_URL")
            .unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string());
        let model = std::env::var("OLLAMA_MODEL")
            .unwrap_or_else(|_| "codellama".to_string());
        
        Self {
            client: Arc::new(Mutex::new(Some(reqwest::Client::new()))),
            api_url,
            model,
        }
    }

    pub async fn get_ai_suggestions(&self, context: &str, history: &[String]) -> Vec<Suggestion> {
        let system_prompt = format!(
            "You are an expert shell command assistant. Given the user's partial input and command history, suggest the most likely next commands or completions.

Rules:
1. Provide only shell commands, no explanations
2. Each suggestion should be a complete, runnable command
3. Consider the context and history for relevance
4. Return 3-5 suggestions, one per line
5. If the input is incomplete, suggest completions
6. If the input is complete, suggest logical next commands

User's partial input: '{}'
Recent command history: {:?}

Suggestions:",
            context,
            history.iter().take(5).collect::<Vec<_>>()
        );

        let request_body = serde_json::json!({
            "model": self.model,
            "prompt": system_prompt,
            "stream": false,
            "options": {
                "temperature": 0.3,
                "top_p": 0.9,
                "max_tokens": 200
            }
        });

        if let Some(client) = &*self.client.lock().unwrap() {
            match client.post(&self.api_url)
                .json(&request_body)
                .timeout(Duration::from_secs(5))
                .send()
                .await
            {
                Ok(res) => {
                    if res.status().is_success() {
                        if let Ok(ai_response) = res.json::<serde_json::Value>().await {
                            if let Some(response_text) = ai_response["response"].as_str() {
                                return self.parse_ai_response(response_text, context);
                            }
                        }
                    }
                }
                Err(e) => {
                    log::warn!("AI completion request failed: {}", e);
                }
            }
        }

        Vec::new()
    }

    fn parse_ai_response(&self, response: &str, context: &str) -> Vec<Suggestion> {
        let mut suggestions = Vec::new();
        
        for line in response.lines() {
            let line = line.trim();
            if !line.is_empty() && !line.starts_with('#') && !line.starts_with("//") {
                let command = line.split_whitespace().collect::<Vec<_>>().join(" ");
                if !command.is_empty() {
                    suggestions.push(Suggestion {
                        display: command.clone(),
                        replacement: command,
                        description: Some("AI suggested".to_string()),
                        suggestion_type: SuggestionType::AiGenerated,
                        confidence: 0.6,
                    });
                }
            }
        }
        
        suggestions
    }
}

/// The central manager that holds all completion logic.
pub struct CompletionManager {
    specs: HashMap<String, Box<dyn Completer + Send + Sync>>,
    file_completer: FilePathCompleter,
    ai_completer: AiCompleter,
    matcher: SkimMatcherV2,
    history: Vec<String>,
    suggestion_cache: Arc<Mutex<HashMap<String, (Vec<Suggestion>, std::time::Instant)>>>,
}

impl CompletionManager {
    pub fn new() -> Self {
        let mut specs: HashMap<String, Box<dyn Completer + Send + Sync>> = HashMap::new();
        
        // Enhanced command specs with descriptions
        specs.insert("git".to_string(), Box::new(CommandSpec {
            subcommands: vec![
                "add", "branch", "checkout", "commit", "fetch", "pull", "push", "status",
                "log", "diff", "merge", "rebase", "stash", "tag", "remote", "clone"
            ].into_iter().map(String::from).collect(),
            flags: vec![
                "--version", "--help", "--verbose", "--quiet", "--dry-run",
                "-a", "-m", "-p", "-v", "-q", "-n"
            ].into_iter().map(String::from).collect(),
            description: "Git version control".to_string(),
        }));
        
        specs.insert("docker".to_string(), Box::new(CommandSpec {
            subcommands: vec![
                "build", "images", "ps", "pull", "push", "run", "stop", "start",
                "exec", "logs", "inspect", "rm", "rmi", "network", "volume"
            ].into_iter().map(String::from).collect(),
            flags: vec![
                "--version", "--help", "--verbose", "--quiet", "--force",
                "-v", "-q", "-f", "-t", "-d", "-p"
            ].into_iter().map(String::from).collect(),
            description: "Docker container management".to_string(),
        }));

        specs.insert("cargo".to_string(), Box::new(CommandSpec {
            subcommands: vec![
                "build", "run", "test", "check", "clippy", "fmt", "doc", "publish",
                "add", "remove", "update", "search", "install", "uninstall"
            ].into_iter().map(String::from).collect(),
            flags: vec![
                "--version", "--help", "--verbose", "--quiet", "--release",
                "-v", "-q", "--release", "--debug", "--bin", "--lib"
            ].into_iter().map(String::from).collect(),
            description: "Rust package manager".to_string(),
        }));

        Self {
            specs,
            file_completer: FilePathCompleter,
            ai_completer: AiCompleter::new(),
            matcher: SkimMatcherV2::default(),
            history: Vec::new(),
            suggestion_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Add a command to history for context
    pub fn add_to_history(&mut self, command: String) {
        self.history.push(command);
        if self.history.len() > 100 {
            self.history.remove(0);
        }
    }

    /// The main entry point for getting suggestions.
    pub fn get_suggestions(&self, line: &str, cursor_pos: usize) -> Vec<Suggestion> {
        let text_before_cursor = &line[..cursor_pos];
        let words: Vec<&str> = text_before_cursor.split_whitespace().collect();
        
        let command = words.get(0).cloned().unwrap_or("");
        let current_word = if text_before_cursor.ends_with(char::is_whitespace) { "" } else { words.last().cloned().unwrap_or("") };

        let mut all_suggestions = Vec::new();

        // 1. Command completion (highest priority)
        if words.len() <= 1 {
            for (cmd_name, _) in &self.specs {
                if cmd_name.starts_with(current_word) {
                    all_suggestions.push(Suggestion {
                        display: cmd_name.clone(),
                        replacement: cmd_name.clone(),
                        description: Some("Command".to_string()),
                        suggestion_type: SuggestionType::Command,
                        confidence: 0.95,
                    });
                }
            }
        } else if let Some(spec) = self.specs.get(command) {
            // 2. Command-specific completions
            all_suggestions.extend(spec.suggest(current_word));
        } else {
            // 3. File path completion for unknown commands
            all_suggestions.extend(self.file_completer.suggest(current_word));
        }

        // 4. History-based suggestions
        for hist_cmd in &self.history {
            if hist_cmd.starts_with(&text_before_cursor) && hist_cmd != text_before_cursor {
                let suffix = &hist_cmd[text_before_cursor.len()..];
                if !suffix.is_empty() {
                    all_suggestions.push(Suggestion {
                        display: hist_cmd.clone(),
                        replacement: hist_cmd.clone(),
                        description: Some("From history".to_string()),
                        suggestion_type: SuggestionType::History,
                        confidence: 0.85,
                    });
                }
            }
        }

        // 5. Fuzzy filter and sort
        let mut scored: Vec<(i64, Suggestion)> = all_suggestions.into_iter()
            .filter_map(|s| {
                let score = self.matcher.fuzzy_match(&s.display, current_word).unwrap_or(0);
                Some((score, s))
            })
            .collect();
        
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        
        // Return top suggestions, prioritizing by type and confidence
        let mut result: Vec<Suggestion> = scored.into_iter().map(|(_, s)| s).collect();
        result.sort_by(|a, b| {
            // Sort by confidence first, then by type priority
            b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    let type_priority = |t: &SuggestionType| match t {
                        SuggestionType::Command => 0,
                        SuggestionType::Subcommand => 1,
                        SuggestionType::Flag => 2,
                        SuggestionType::History => 3,
                        SuggestionType::FilePath => 4,
                        SuggestionType::AiGenerated => 5,
                        SuggestionType::Argument => 6,
                        SuggestionType::Workflow => 7,
                    };
                    type_priority(&a.suggestion_type).cmp(&type_priority(&b.suggestion_type))
                })
        });

        result.truncate(10); // Limit to top 10 suggestions
        result
    }

    /// Get AI-powered suggestions asynchronously
    pub async fn get_ai_suggestions(&self, line: &str, cursor_pos: usize) -> Vec<Suggestion> {
        let text_before_cursor = &line[..cursor_pos];
        
        // Check cache first
        {
            let cache = self.suggestion_cache.lock().unwrap();
            if let Some((cached_suggestions, timestamp)) = cache.get(text_before_cursor) {
                if timestamp.elapsed() < Duration::from_secs(30) {
                    return cached_suggestions.clone();
                }
            }
        }

        // Get AI suggestions
        let ai_suggestions = self.ai_completer.get_ai_suggestions(text_before_cursor, &self.history).await;
        
        // Cache the results
        {
            let mut cache = self.suggestion_cache.lock().unwrap();
            cache.insert(text_before_cursor.to_string(), (ai_suggestions.clone(), std::time::Instant::now()));
        }

        ai_suggestions
    }

    /// Get all suggestions (traditional + AI) asynchronously
    pub async fn get_all_suggestions(&self, line: &str, cursor_pos: usize) -> Vec<Suggestion> {
        let mut suggestions = self.get_suggestions(line, cursor_pos);
        
        // Add AI suggestions if we have few traditional suggestions
        if suggestions.len() < 3 {
            let ai_suggestions = self.get_ai_suggestions(line, cursor_pos).await;
            suggestions.extend(ai_suggestions);
        }

        // Remove duplicates and sort
        suggestions.sort_by(|a, b| {
            b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        suggestions.dedup_by(|a, b| a.replacement == b.replacement);
        suggestions.truncate(15);
        
        suggestions
    }
} 