use aws_sdk_dynamodb as dynamodb;
use aws_sdk_dynamodb::types;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod table;

type DynamoResult<T = Box<dyn show::Show>> = std::result::Result<T, dynamodb::Error>;

/// Amazon DynamoDB is a fully managed NoSQL database service
///
/// Amazon DynamoDB is a fully managed NoSQL database service that provides
/// fast and predictable performance with seamless scalability. DynamoDB
/// lets you offload the administrative burdens of operating and scaling a
/// distributed database, so that you don't have to worry about hardware
/// provisioning, setup and configuration, replication, software patching,
/// or cluster scaling.
///
/// With DynamoDB, you can create database tables that can store and
/// retrieve any amount of data, and serve any level of request traffic.
/// You can scale up or scale down your tables' throughput capacity without
/// downtime or performance degradation, and use the Amazon Web Services
/// Management Console to monitor resource utilization and performance
/// metrics.
///
/// DynamoDB automatically spreads the data and traffic for your tables
/// over a sufficient number of servers to handle your throughput and
/// storage requirements, while maintaining consistent and fast
/// performance. All of your data is stored on solid state disks (SSDs) and
/// automatically replicated across multiple Availability Zones in an
/// Amazon Web Services Region, providing built-in high availability and
/// data durability.
#[derive(Debug, Subcommand)]
#[command(verbatim_doc_comment)]
pub enum DynamoDb {
    CreateTable(table::CreateTable),
    DeleteTable(table::DeleteTable),
    DescribeTable(table::DescribeTable),
    ListTables(table::ListTables),
}

impl DynamoDb {
    async fn execute(self, config: &Config) -> DynamoResult {
        match self {
            Self::CreateTable(create_table) => create_table.execute(config).await,
            Self::DeleteTable(delete_table) => delete_table.execute(config).await,
            Self::DescribeTable(describe_table) => describe_table.execute(config).await,
            Self::ListTables(list_tables) => list_tables.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<dynamodb::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
