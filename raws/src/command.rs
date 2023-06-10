use account::Account;
use config::Config;
use dynamodb::DynamoDb;
use eks::Eks;
use iam::Iam;
use sso::Sso;
use sts::Sts;

use super::*;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(subcommand)]
    Account(Account),

    #[command(subcommand)]
    Dynamodb(DynamoDb),

    Ec2,

    #[command(subcommand)]
    Eks(Eks),

    #[command(subcommand)]
    Iam(Iam),

    #[command(subcommand)]
    Sso(Sso),

    /// Security Token Service (STS) operations
    #[command(subcommand)]
    Sts(Sts),
}

impl Command {
    pub(crate) async fn dispatch(self, config: Config) -> miette::Result<()> {
        match self {
            Self::Account(account) => account.dispatch(config).await?,
            Self::Dynamodb(dynamo) => dynamo.dispatch(config).await?,
            Self::Ec2 => todo!(),
            Self::Eks(eks) => eks.dispatch(config).await?,
            Self::Iam(iam) => iam.dispatch(config).await?,
            Self::Sso(sso) => sso.dispatch(config).await?,
            Self::Sts(sts) => sts.dispatch(config).await?,
        }
        Ok(())
    }
}
