use super::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Use a specific profile from your credential file.
    #[arg(long, global = true)]
    profile: Option<String>,

    /// The region to use. Overrides config/env settings.
    #[arg(long, global = true)]
    region: Option<String>,

    /// The formatting style for the command output.
    #[arg(long, global = true, value_enum)]
    output: Option<config::Output>,

    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub(crate) async fn execute(self) -> miette::Result<()> {
        let config = config::Config::new(self.profile, self.region, self.output).await;
        self.command.dispatch(config).await
    }
}
