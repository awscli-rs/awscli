use super::*;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    /// Debug data representation
    #[arg(long, global = true, display_order = 1000010)]
    debug: bool,

    /// Override command's default URL with the given URL.
    #[arg(long, global = true, display_order = 1000020)]
    endpoint_url: Option<String>,

    /// By default, the AWS CLI uses SSL when communicating with AWS services.
    /// For each SSL connection, the AWS CLI will verify SSL certificates. This
    /// option overrides the default behavior of verifying SSL certificates.
    #[arg(long, global = true, display_order = 1000030)]
    no_verify_ssl: bool,

    /// Disable automatic pagination.
    #[arg(long, global = true, display_order = 1000040)]
    no_paginate: bool,

    /// The formatting style for the command output.
    #[arg(long, global = true, value_enum, display_order = 1000050)]
    output: Option<config::Output>,

    /// Use a specific profile from your credential file.
    #[arg(long, global = true, display_order = 1000060)]
    profile: Option<String>,

    /// The region to use. Overrides config/env settings.
    #[arg(long, global = true, display_order = 1000070)]
    region: Option<String>,

    #[command(subcommand)]
    command: Command,
}

impl Cli {
    pub(crate) async fn execute(self) -> miette::Result<()> {
        let config = config::Config::new(
            self.debug,
            self.no_paginate,
            self.endpoint_url,
            self.profile,
            self.region,
            self.output,
        )
        .await;
        self.command.dispatch(config).await
    }
}
