use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, EnvFilter};

use cli::Cli;
use command::Command;

mod cli;
mod command;

#[tokio::main]
async fn main() -> miette::Result<()> {
    fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .with_timer(fmt::time::UtcTime::rfc_3339())
        .init();

    Cli::parse().execute().await
}
