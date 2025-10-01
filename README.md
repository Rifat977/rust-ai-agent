# ⚡ Rust AI Agent

A blazing-fast AI agent powered by Rust with web scraping capabilities, demonstrating high-performance LLM integration.

## ✨ Features

- **AI Agent Framework** - Intelligent agent that can reason and use tools
- **High-Performance Web Scraper** - Extract content from websites at lightning speed
- **LLM Integration** - Supports OpenAI and Anthropic APIs
- **Interactive CLI** - Beautiful command-line interface

## 🚀 Quick Start

### Setup

```bash
# Install dependencies
cargo build --release

# Set your API key
export OPENAI_API_KEY="your-key-here"
# OR
export ANTHROPIC_API_KEY="your-key-here"
```

### Usage

```bash
# Ask the agent to scrape a website
cargo run --release -- run "scrape https://example.com and summarize the content"

# Direct scraping (no LLM, pure speed)
cargo run --release -- scrape https://example.com

# Interactive mode
cargo run --release -- interactive
```

## 📄 License

MIT License - Feel free to use this as a starting point for your own Rust AI experiments!
