use aws_sdk_dynamodb as dynamo;
use clap::Subcommand;
use miette::IntoDiagnostic;

use config::Config;

#[derive(Debug, Subcommand)]
pub enum DynamoDb {
    CreateTable {
        /// The name of the table to create.
        #[arg(long)]
        table_name: String,
    },
    DeleteTable,
    DescribeTable,
}

impl DynamoDb {
    pub async fn dispatch(self, config: Config) -> miette::Result<()> {
        let client = dynamo::Client::new(config.config());
        match self {
            Self::CreateTable { table_name } => create_table(client, table_name).await,
            Self::DeleteTable => Ok(()),
            Self::DescribeTable => Ok(()),
        }
    }
}

async fn create_table(client: dynamo::Client, table_name: String) -> miette::Result<()> {
    let output = client
        .create_table()
        .table_name(table_name)
        .send()
        .await
        .into_diagnostic()?;

    println!("{output:?}");

    Ok(())
}
