use super::*;

#[derive(Debug, Args)]
pub struct DescribeTable {
    /// The name of the table to describe.
    #[arg(long)]
    table_name: String,
}

impl DescribeTable {
    pub async fn execute(self, client: dynamo::Client) -> DynamoResult {
        let output = client
            .describe_table()
            .table_name(self.table_name)
            .send()
            .await?;
        println!("{output:?}");
        Ok(())
    }
}
