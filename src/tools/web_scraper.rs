use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use scraper::{Html, Selector};
use serde_json::{json, Value};

pub struct WebScraperTool {
    client: reqwest::Client,
}

impl WebScraperTool {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::builder()
                .user_agent("Mozilla/5.0 (compatible; RustAIAgent/1.0)")
                .timeout(std::time::Duration::from_secs(10))
                .build()
                .unwrap(),
        }
    }

    async fn scrape_url(&self, url: &str) -> Result<(String, String, Vec<String>)> {
        let response = self.client.get(url).send().await?;
        let html_content = response.text().await?;
        let document = Html::parse_document(&html_content);

        let title_selector = Selector::parse("title").unwrap();
        let title = document
            .select(&title_selector)
            .next()
            .map(|el| el.inner_html())
            .unwrap_or_else(|| "No title".to_string());

        let text_selectors = ["p", "article", "main", "section"];
        let mut text_content = Vec::new();
        
        for selector_str in text_selectors {
            if let Ok(selector) = Selector::parse(selector_str) {
                for element in document.select(&selector) {
                    let text = element.text().collect::<Vec<_>>().join(" ");
                    if !text.trim().is_empty() {
                        text_content.push(text.trim().to_string());
                    }
                }
            }
        }

        let link_selector = Selector::parse("a[href]").unwrap();
        let links: Vec<String> = document
            .select(&link_selector)
            .filter_map(|el| el.value().attr("href"))
            .map(|s| s.to_string())
            .collect();

        let full_text = text_content.join("\n");
        Ok((title, full_text, links))
    }
}

#[async_trait]
impl Tool for WebScraperTool {
    fn name(&self) -> &str {
        "web_scraper"
    }

    fn description(&self) -> &str {
        "Fetch and extract content from web pages at lightning speed. Input: {\"url\": \"https://example.com\"}"
    }

    async fn execute(&self, input: Value) -> Result<ToolResult> {
        let url = input
            .get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("Missing 'url' parameter"))?;

        let (title, text, links) = self.scrape_url(url).await?;

        let word_count = text.split_whitespace().count();
        let link_count = links.len();

        Ok(ToolResult::with_metadata(
            json!({
                "title": title,
                "content": text,
                "links": links.iter().take(10).collect::<Vec<_>>(),
                "summary": {
                    "word_count": word_count,
                    "link_count": link_count,
                }
            }),
            json!({
                "performance": "native_rust_scraping",
                "features": ["parallel_parsing", "low_memory"]
            }),
        ))
    }
}

