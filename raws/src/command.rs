use account::Account;
use config::Config;
use dynamodb::DynamoDb;
use ebs::Ebs;
use ec2::Ec2;
use eks::Eks;
use iam::Iam;
use pricing::Pricing;
use sso::Sso;
use sts::Sts;

use super::*;

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    #[command(subcommand)]
    Account(Account),

    #[command(subcommand)]
    Dynamodb(DynamoDb),

    #[command(subcommand)]
    Ebs(Ebs),

    #[command(subcommand)]
    Ec2(Ec2),

    #[command(subcommand)]
    Eks(Eks),

    #[command(subcommand)]
    Iam(Iam),

    #[command(subcommand)]
    Pricing(Pricing),

    #[command(subcommand)]
    Sso(Sso),

    #[command(subcommand)]
    Sts(Sts),
}

impl Command {
    pub(crate) async fn dispatch(self, config: Config) -> miette::Result<()> {
        match self {
            Self::Account(account) => account.dispatch(config).await?,
            Self::Dynamodb(dynamo) => dynamo.dispatch(config).await?,
            Self::Ec2(ec2) => ec2.dispatch(config).await?,
            Self::Ebs(ebs) => ebs.dispatch(config).await?,
            Self::Eks(eks) => eks.dispatch(config).await?,
            Self::Iam(iam) => iam.dispatch(config).await?,
            Self::Pricing(pricing) => pricing.dispatch(config).await?,
            Self::Sso(sso) => sso.dispatch(config).await?,
            Self::Sts(sts) => sts.dispatch(config).await?,
        }
        Ok(())
    }
}
