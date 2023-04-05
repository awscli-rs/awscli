use super::*;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub async fn execute(self) -> miette::Result<()> {
        let config = config::Config::load().await;
        self.command.dispatch(config).await
    }
}
