use anyhow::Result;
use clap::{Parser, Subcommand};
use colored::Colorize;
use rust_ai_agents::{tools::{Tool, WebScraperTool}, Agent};
use rust_ai_agents::llm::LLMClient;
use serde_json::json;
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "rust-ai-agent")]
#[command(about = "⚡ Rust-powered AI Agent with web scraping capabilities", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Run {
        query: String,
    },
    Scrape {
        url: String,
    },
    Interactive,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    
    let cli = Cli::parse();

    match cli.command {
        Commands::Run { query } => {
            let agent = create_agent().await?;
            let response = agent.run(&query).await?;
            
            println!("{}", "─".repeat(60).bright_black());
            println!("{}", "📝 FINAL RESPONSE".green().bold());
            println!("{}", "─".repeat(60).bright_black());
            println!("{}", response);
            println!("{}\n", "─".repeat(60).bright_black());
        }
        Commands::Scrape { url } => {
            scrape_url(&url).await?;
        }
        Commands::Interactive => {
            run_interactive_mode().await?;
        }
    }

    Ok(())
}

async fn create_agent() -> Result<Agent> {
    
    let llm = LLMClient::from_env()?;
    let mut agent = Agent::new(llm);

    agent.register_tool(Arc::new(WebScraperTool::new()));

    Ok(agent)
}

async fn scrape_url(url: &str) -> Result<()> {
    use std::time::Instant;

    println!("\n{}", "═".repeat(60).cyan());
    println!("{} {}", "🌐 SCRAPING URL".cyan().bold(), url);
    println!("{}", "═".repeat(60).cyan());
    
    let scraper = WebScraperTool::new();
    let start = Instant::now();
    
    let result = scraper.execute(json!({"url": url})).await?;
    let duration = start.elapsed();

    println!("\n{}", "RESULT:".yellow().bold());
    println!("{}", serde_json::to_string_pretty(&result.output)?);
    println!(
        "\n{} Scraped in {:.2}ms\n",
        "⚡".green().bold(),
        duration.as_secs_f64() * 1000.0
    );

    Ok(())
}

async fn run_interactive_mode() -> Result<()> {
    use std::io::{self, Write};

    let agent = create_agent().await?;
    
    println!("{}", "═".repeat(60).cyan());
    println!("{}", "💬 INTERACTIVE MODE".cyan().bold());
    println!("{}", "═".repeat(60).cyan());
    println!("{} Type 'exit' to quit", "ℹ".blue().bold());
    println!("{} Try: 'scrape https://example.com' or ask questions", "💡".yellow());
    println!();

    loop {
        print!("{} ", "Query:".yellow().bold());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let query = input.trim();
        if query.is_empty() {
            continue;
        }
        
        if query.eq_ignore_ascii_case("exit") || query.eq_ignore_ascii_case("quit") {
            println!("\n{} {}\n", "👋".cyan(), "Goodbye!".cyan().bold());
            break;
        }

        if query.starts_with("scrape ") {
            if let Some(url) = query.strip_prefix("scrape ") {
                if let Err(e) = scrape_url(url.trim()).await {
                    println!("{} {}\n", "Error:".red().bold(), e);
                }
                continue;
            }
        }

        match agent.run(query).await {
            Ok(response) => {
                println!("\n{}", "─".repeat(60).bright_black());
                println!("{}", "RESPONSE".green().bold());
                println!("{}", "─".repeat(60).bright_black());
                println!("{}", response);
                println!("{}\n", "─".repeat(60).bright_black());
            }
            Err(e) => {
                println!("\n{} {}\n", "❌ Error:".red().bold(), e);
            }
        }
    }

    Ok(())
}
