# Rust AI Agent

A high-performance AI agent framework built with Rust, featuring web scraping and LLM integration.

## Features

- AI agent with tool-based reasoning
- Web scraper with content extraction
- OpenAI and Anthropic API support
- Interactive CLI with colored output

## Setup

**Requirements:**
- Rust 1.70+
- OpenAI or Anthropic API key

**Installation:**

```bash
cargo build --release
```

**Configuration:**

Create a `.env` file or export environment variables:

```bash
export OPENAI_API_KEY="your-key-here"
# or
export ANTHROPIC_API_KEY="your-key-here"
```

## Usage

**Run a query:**

```bash
cargo run --quiet -- run "scrape https://example.com and summarize"
```

**Direct web scraping:**

```bash
cargo run --quiet -- scrape https://example.com
```

**Interactive mode:**

```bash
cargo run --quiet -- interactive
```

