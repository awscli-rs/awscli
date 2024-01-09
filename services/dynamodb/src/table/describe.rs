use super::*;

#[derive(Debug, Args)]
pub struct DescribeTable {
    /// The name of the table to describe.
    #[arg(long)]
    table_name: String,
}

impl DescribeTable {
    pub(crate) async fn execute(self, config: &Config) -> DynamoResult {
        let table = config
            .dynamodb()
            .describe_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table;
        Ok(Box::new(table))
    }
}
