use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum DynamoDb {
    CreateTable,
    DeleteTable,
    DescribeTable,
}

impl DynamoDb {
    pub async fn dispatch(self) -> miette::Result<()> {
        match self {
            Self::CreateTable => Ok(()),
            Self::DeleteTable => Ok(()),
            Self::DescribeTable => Ok(()),
        }
    }
}
