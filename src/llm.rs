use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone)]
pub struct LLMClient {
    client: reqwest::Client,
    api_key: String,
    model: String,
    provider: LLMProvider,
}

#[derive(Clone)]
enum LLMProvider {
    OpenAI,
    Anthropic,
}

#[derive(Serialize)]
struct OpenAIRequest {
    model: String,
    messages: Vec<OpenAIMessage>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct OpenAIMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAIResponse {
    choices: Vec<OpenAIChoice>,
}

#[derive(Deserialize)]
struct OpenAIChoice {
    message: OpenAIMessage,
}

#[derive(Serialize)]
struct AnthropicRequest {
    model: String,
    max_tokens: u32,
    messages: Vec<AnthropicMessage>,
    system: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct AnthropicMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct AnthropicResponse {
    content: Vec<AnthropicContent>,
}

#[derive(Deserialize)]
struct AnthropicContent {
    text: String,
}

impl LLMClient {
    pub fn from_env() -> Result<Self> {
        if let Ok(api_key) = env::var("OPENAI_API_KEY") {
            return Ok(Self::new_openai(api_key));
        }

        if let Ok(api_key) = env::var("ANTHROPIC_API_KEY") {
            return Ok(Self::new_anthropic(api_key));
        }

        Err(anyhow::anyhow!(
            "No API key found. Set OPENAI_API_KEY or ANTHROPIC_API_KEY environment variable"
        ))
    }

    pub fn new_openai(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            model: "gpt-4o-mini".to_string(),
            provider: LLMProvider::OpenAI,
        }
    }

    pub fn new_anthropic(api_key: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key,
            model: "claude-3-5-sonnet-20241022".to_string(),
            provider: LLMProvider::Anthropic,
        }
    }

    pub async fn chat(&self, system: &str, user_message: &str) -> Result<String> {
        match self.provider {
            LLMProvider::OpenAI => self.chat_openai(system, user_message).await,
            LLMProvider::Anthropic => self.chat_anthropic(system, user_message).await,
        }
    }

    async fn chat_openai(&self, system: &str, user_message: &str) -> Result<String> {
        let request = OpenAIRequest {
            model: self.model.clone(),
            messages: vec![
                OpenAIMessage {
                    role: "system".to_string(),
                    content: system.to_string(),
                },
                OpenAIMessage {
                    role: "user".to_string(),
                    content: user_message.to_string(),
                },
            ],
            max_tokens: 1000,
            temperature: 0.7,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        let response_data: OpenAIResponse = response.json().await?;
        
        Ok(response_data
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .unwrap_or_else(|| "No response".to_string()))
    }

    async fn chat_anthropic(&self, system: &str, user_message: &str) -> Result<String> {
        let request = AnthropicRequest {
            model: self.model.clone(),
            max_tokens: 1000,
            system: Some(system.to_string()),
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: user_message.to_string(),
            }],
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;

        let response_data: AnthropicResponse = response.json().await?;
        
        Ok(response_data
            .content
            .first()
            .map(|c| c.text.clone())
            .unwrap_or_else(|| "No response".to_string()))
    }
}

