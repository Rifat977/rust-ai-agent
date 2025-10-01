use crate::llm::LLMClient;
use crate::tools::{Tool, ToolResult};
use anyhow::Result;
use colored::Colorize;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

pub struct Agent {
    llm: Arc<LLMClient>,
    tools: HashMap<String, Arc<dyn Tool>>,
}

impl Agent {
    pub fn new(llm: LLMClient) -> Self {
        Self {
            llm: Arc::new(llm),
            tools: HashMap::new(),
        }
    }

    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.insert(tool.name().to_string(), tool);
    }

    pub async fn execute_tool(&self, tool_name: &str, input: Value) -> Result<ToolResult> {
        let start = Instant::now();
        
        let tool = self
            .tools
            .get(tool_name)
            .ok_or_else(|| anyhow::anyhow!("Tool '{}' not found", tool_name))?;

        let result = tool.execute(input).await?;
        let duration = start.elapsed();
        
        println!(
            "{} Tool '{}' executed in {:.2}ms",
            "⚡".green().bold(),
            tool_name.cyan(),
            duration.as_secs_f64() * 1000.0
        );

        Ok(result)
    }

    pub async fn run(&self, query: &str) -> Result<String> {
        println!("{}", "─".repeat(60));
        println!("🔍 {}: {}", "Processing Query".bold(), query);
        println!("{}\n", "─".repeat(60));

        let tool_descriptions: Vec<String> = self
            .tools
            .values()
            .map(|t| format!("- {}: {}", t.name(), t.description()))
            .collect();

        let system_prompt = format!(
            "You are a helpful AI agent with access to the following tools:\n{}\n\n\
            When you need to use a tool, respond with JSON in this format:\n\
            {{\"tool\": \"tool_name\", \"input\": {{...input_data...}}}}\n\n\
            After getting tool results, provide a natural language response to the user.",
            tool_descriptions.join("\n")
        );

        let llm_response = self.llm.chat(&system_prompt, query).await?;
        
        if let Ok(tool_call) = serde_json::from_str::<Value>(&llm_response) {
            if let (Some(tool_name), Some(input)) = (
                tool_call.get("tool").and_then(|v| v.as_str()),
                tool_call.get("input"),
            ) {
                
                let tool_result = self.execute_tool(tool_name, input.clone()).await?;
                
                let result_prompt = format!(
                    "User query: {}\n\nThe tool '{}' returned: {}\n\nBased on this result, answer the user's query.",
                    query, tool_name, tool_result.output
                );
                
                let final_response = self.llm.chat(&system_prompt, &result_prompt).await?;
                return Ok(final_response);
            }
        }

        Ok(llm_response)
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }
}

