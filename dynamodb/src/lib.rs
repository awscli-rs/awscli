use aws_sdk_dynamodb as dynamo;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

type DynamoResult<T> = std::result::Result<T, dynamo::Error>;

mod table;

#[derive(Debug, Subcommand)]
pub enum DynamoDb {
    CreateTable(table::CreateTable),
    DeleteTable(table::DeleteTable),
    DescribeTable(table::DescribeTable),
    ListTables(table::ListTables),
}

impl DynamoDb {
    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<dynamo::Error>> {
        let client = dynamo::Client::new(config.config());
        match self {
            Self::CreateTable(create_table) => create_table
                .execute(client)
                .await
                .map(|output| config.output(output))?,
            Self::DeleteTable(delete_table) => delete_table
                .execute(client)
                .await
                .map(|output| config.output(output))?,
            Self::DescribeTable(describe_table) => describe_table
                .execute(client)
                .await
                .map(|output| config.output(output))?,
            Self::ListTables(list_tables) => list_tables
                .execute(client)
                .await
                .map(|output| config.output(output))?,
        }
        Ok(())
    }
}
