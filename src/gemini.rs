use anyhow::{Context, Result};
use dotenv::dotenv;
use log::{debug, error, info};
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use thiserror::Error;

// Error types for the Gemini API
#[derive(Error, Debug)]
pub enum GeminiError {
    #[error("API key not found. Set GEMINI_API_KEY environment variable.")]
    ApiKeyNotFound,
    
    #[error("API request failed: {0}")]
    RequestFailed(String),
    
    #[error("Failed to parse API response: {0}")]
    ParseError(String),
}

// Request and response structures for the Gemini API
#[derive(Serialize, Debug)]
struct GeminiRequest {
    contents: Vec<GeminiContent>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiContent {
    parts: Vec<GeminiPart>,
}

#[derive(Serialize, Deserialize, Debug)]
struct GeminiPart {
    text: String,
}

#[derive(Deserialize, Debug)]
struct GeminiResponse {
    candidates: Vec<GeminiCandidate>,
}

#[derive(Deserialize, Debug)]
struct GeminiCandidate {
    content: GeminiContent,
}

// Main client for interacting with the Gemini API
pub struct GeminiClient {
    api_key: String,
    client: Client,
    demo_mode: bool,
}

impl GeminiClient {
    // Create a new Gemini client
    pub fn new() -> Result<Self> {
        // Load environment variables from .env file
        dotenv().ok();
        
        // Check for demo mode
        let demo_mode = env::var("DSHPC_DEMO_MODE").unwrap_or_default() == "1";
        
        // If not in demo mode, get API key from environment variables
        let api_key = if !demo_mode {
            env::var("GEMINI_API_KEY")
                .map_err(|_| GeminiError::ApiKeyNotFound)?
        } else {
            info!("Running in demo mode - API calls will be simulated");
            "demo_mode".to_string()
        };
        
        let client = Client::new();
        
        Ok(Self { api_key, client, demo_mode })
    }
    
    // Generate code from a natural language prompt
    pub fn generate_code(&self, prompt: &str) -> Result<String> {
        debug!("Generating code with prompt: {}", prompt);
        
        // If in demo mode, return predefined examples
        if self.demo_mode {
            return Ok(self.get_demo_code(prompt));
        }
        
        // Prepare the request
        let request = GeminiRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: prompt.to_string(),
                }],
            }],
        };
        
        // Send the request to the Gemini API
        let url = format!(
            "https://generativelanguage.googleapis.com/v1/models/gemini-2.0-flash:generateContent?key={}",
            self.api_key
        );
        
        let response = self.client
            .post(&url)
            .json(&request)
            .send()
            .with_context(|| "Failed to send request to Gemini API")?;
        
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_else(|_| "Unknown error".to_string());
            error!("API request failed with status {}: {}", status, error_text);
            return Err(GeminiError::RequestFailed(error_text).into());
        }
        
        // Parse the response
        let gemini_response: GeminiResponse = response
            .json()
            .with_context(|| "Failed to parse Gemini API response")?;
        
        // Extract the generated code
        let candidate = gemini_response.candidates
            .first()
            .ok_or_else(|| GeminiError::ParseError("No candidates in response".to_string()))?;
        
        let text = candidate.content.parts
            .first()
            .ok_or_else(|| GeminiError::ParseError("No parts in content".to_string()))?
            .text
            .clone();
        
        // Extract only the code portion from the response
        let code = self.extract_code(&text);
        debug!("Generated code: {}", code);
        
        Ok(code)
    }
    
    // Extract code from the Gemini response
    fn extract_code(&self, text: &str) -> String {
        // This is a simple implementation that looks for code blocks
        // A more sophisticated version would use regex or a parser
        if let Some(start) = text.find("```") {
            if let Some(end) = text[start + 3..].find("```") {
                let code_block = &text[start + 3..start + 3 + end];
                // Remove language identifier if present
                if let Some(newline) = code_block.find('\n') {
                    return code_block[newline + 1..].trim().to_string();
                }
                return code_block.trim().to_string();
            }
        }
        
        // If no code block markers are found, return the full text
        text.to_string()
    }
    
    // Get demo code for testing without API key
    fn get_demo_code(&self, prompt: &str) -> String {
        // Check for sample files
        if prompt.contains("Hello, World!") || prompt.contains("sum of two integers") {
            if let Ok(code) = fs::read_to_string("dshpc/examples/hello_world.cpp") {
                return code;
            }
            return include_str!("../../examples/hello_world.cpp").to_string();
        } else if prompt.contains("array of integers") || prompt.contains("sum of all elements") {
            if let Ok(code) = fs::read_to_string("dshpc/examples/array_sum.rs") {
                return code;
            }
            return include_str!("../../examples/array_sum.rs").to_string();
        } else if prompt.contains("Fibonacci") {
            if let Ok(code) = fs::read_to_string("dshpc/examples/fibonacci.asm") {
                return code;
            }
            return include_str!("../../examples/fibonacci.asm").to_string();
        } else {
            // Default example - Hello World
            let code = "#include <iostream>\n\nint main() {\n    std::cout << \"Hello, World!\" << std::endl;\n    return 0;\n}";
            return code.to_string();
        }
    }
} 