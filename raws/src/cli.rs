use super::*;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn execute(self) -> miette::Result<()> {
        self.command.dispatch().await
    }
}
