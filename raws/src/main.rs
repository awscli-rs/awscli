use clap::{Parser, Subcommand};

use cli::Cli;
use command::Command;

mod cli;
mod command;

#[tokio::main]
async fn main() -> miette::Result<()> {
    Cli::parse().execute().await
}
