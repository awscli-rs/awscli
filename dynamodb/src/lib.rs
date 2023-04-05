use aws_sdk_dynamodb as dynamo;

use clap::{Args, Subcommand};

use config::Config;

type DynamoResult<T = ()> = Result<T, dynamo::Error>;

mod table;

#[derive(Debug, Subcommand)]
pub enum DynamoDb {
    CreateTable(table::CreateTable),
    DeleteTable,
    DescribeTable,
}

impl DynamoDb {
    pub async fn dispatch(self, config: Config) -> Result<(), error::Error<dynamo::Error>> {
        let client = dynamo::Client::new(config.config());
        match self {
            Self::CreateTable(create_table) => create_table.execute(client).await?,
            Self::DeleteTable => todo!(),
            Self::DescribeTable => todo!(),
        }
        Ok(())
    }
}
