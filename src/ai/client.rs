use crate::error::{AppResult, AppError};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct AiRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize, Debug)]
struct AiResponse {
    response: String,
    done: bool,
}

/// An example AI client for interacting with a service like Ollama.
pub struct AiClient {
    client: reqwest::Client,
    api_url: String,
}

impl AiClient {
    pub fn new() -> Self {
        // Default to a common local Ollama API endpoint.
        let api_url = std::env::var("OLLAMA_API_URL")
            .unwrap_or_else(|_| "http://localhost:11434/api/generate".to_string());
        
        println!("[AI] Client configured for URL: {}", api_url);

        Self {
            client: reqwest::Client::new(),
            api_url,
        }
    }

    /// Get a command suggestion from the AI based on the user's prompt.
    pub async fn get_command_suggestion(&self, user_prompt: &str) -> AppResult<String> {
        let system_prompt = "You are an expert in shell commands. Given the user's request, provide a single, runnable shell command that accomplishes their goal. Provide only the command, with no explanation or preamble.";
        
        let full_prompt = format!("{}\n\nUser request: '{}'", system_prompt, user_prompt);

        let request_body = AiRequest {
            model: "codellama".to_string(), // A common model for code/command generation
            prompt: full_prompt,
            stream: false,
        };

        // In a real app, you might use a streaming response for better UX.
        let res = self.client.post(&self.api_url)
            .json(&request_body)
            .send()
            .await?;
        
        if !res.status().is_success() {
            let err_body = res.text().await?;
            return Err(AppError::Config(format!("[AI] Error: {}", err_body)));
        }

        let ai_response: AiResponse = res.json().await?;

        // Clean up the response, removing potential quotes or newlines.
        let cleaned_command = ai_response.response.trim().trim_matches('`').to_string();

        Ok(cleaned_command)
    }
}