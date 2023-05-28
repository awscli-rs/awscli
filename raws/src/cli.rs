use super::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Debug data representation
    #[arg(long, global = true, display_order = 1000010)]
    debug: bool,

    /// Use a specific profile from your credential file.
    #[arg(long, global = true, display_order = 1000020)]
    profile: Option<String>,

    /// The region to use. Overrides config/env settings.
    #[arg(long, global = true, display_order = 1000030)]
    region: Option<String>,

    /// The formatting style for the command output.
    #[arg(long, global = true, value_enum, display_order = 1000040)]
    output: Option<config::Output>,

    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub(crate) async fn execute(self) -> miette::Result<()> {
        let config = config::Config::new(self.debug, self.profile, self.region, self.output).await;
        self.command.dispatch(config).await
    }
}
