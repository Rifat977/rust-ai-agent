pub mod web_scraper;

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

#[derive(Debug, Clone)]
pub struct ToolResult {
    pub output: Value,
    pub metadata: Option<Value>,
}

impl ToolResult {
    pub fn new(output: Value) -> Self {
        Self {
            output,
            metadata: None,
        }
    }

    pub fn with_metadata(output: Value, metadata: Value) -> Self {
        Self {
            output,
            metadata: Some(metadata),
        }
    }
}

#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    
    fn description(&self) -> &str;
    
    async fn execute(&self, input: Value) -> Result<ToolResult>;
}

pub use web_scraper::WebScraperTool;
