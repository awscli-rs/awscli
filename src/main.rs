use clap::{Parser, Subcommand};

mod cli;

#[tokio::main]
async fn main() {
    cli::Cli::parse();
}
