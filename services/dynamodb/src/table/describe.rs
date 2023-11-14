use super::*;

#[derive(Debug, Args)]
pub struct DescribeTable {
    /// The name of the table to describe.
    #[arg(long)]
    table_name: String,
}

#[async_trait]
impl Execute for DescribeTable {
    async fn execute(self: Box<Self>, config: &Config) -> DynamoResult {
        let table = config
            .client()
            .describe_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table;
        Ok(Box::new(table))
    }
}
