//! The main entry point for the GitHub Copilot Usage Viewer application.

mod error;
mod github_client;
mod processor;
mod formatter;
mod types;

use clap::Parser;
use error::AppError;
use std::env;

/// A command-line tool to fetch and display GitHub Copilot seat usage for an organization.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The name of your GitHub organization.
    #[arg(short, long)]
    organization: String,
}

/// The main asynchronous function that orchestrates the application's workflow.
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables from a .env file, if it exists.
    dotenv::dotenv().ok();
    let args = Args::parse();

    // Retrieve the GitHub PAT from the environment.
    let token = env::var("GITHUB_PAT").map_err(|_| AppError::MissingToken)?;

    println!("Fetching Copilot usage for organization: {}...", args.organization);

    // Create the API client.
    let client = github_client::create_client()?;
    
    // Fetch, process, and display the usage data.
    match github_client::get_copilot_usage(&client, &args.organization, &token).await {
        Ok(raw_seats) => {
            let processed_seats = processor::process_usage_data(raw_seats);
            formatter::display_usage_table(&processed_seats);
        }
        Err(e) => {
            // Print a user-friendly error message and exit.
            eprintln!("\nError: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
