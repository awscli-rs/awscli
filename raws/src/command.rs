use config::Config;
use dynamodb::DynamoDb;

use super::*;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// DynamoDB operations
    #[command(subcommand)]
    Dynamodb(DynamoDb),
    Ec2,
    Eks,
}

impl Command {
    pub async fn dispatch(self, config: Config) -> miette::Result<()> {
        match self {
            Self::Dynamodb(dynamo) => dynamo.dispatch(config).await,
            Self::Ec2 => Ok(()),
            Self::Eks => Ok(()),
        }
    }
}
