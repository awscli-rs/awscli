use async_trait::async_trait;
use aws_sdk_dynamodb as dynamodb;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod table;

type DynamoResult<T = Box<dyn show::Show>> = std::result::Result<T, dynamodb::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: dynamodb::Client) -> DynamoResult;
}

#[derive(Debug, Subcommand)]
pub enum DynamoDb {
    CreateTable(table::CreateTable),
    DeleteTable(table::DeleteTable),
    DescribeTable(table::DescribeTable),
    ListTables(table::ListTables),
}

impl DynamoDb {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::CreateTable(create_table) => Box::new(create_table),
            Self::DeleteTable(delete_table) => Box::new(delete_table),
            Self::DescribeTable(describe_table) => Box::new(describe_table),
            Self::ListTables(list_tables) => Box::new(list_tables),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<dynamodb::Error>> {
        let client = dynamodb::Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
