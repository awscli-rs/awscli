use super::*;

#[derive(Debug, Args)]
pub struct DeleteTable {
    /// The name of the table to delete.
    #[arg(long)]
    table_name: String,
}

#[async_trait]
impl Execute for DeleteTable {
    async fn execute(self: Box<Self>, client: dynamodb::Client) -> DynamoResult {
        let table = client
            .delete_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table_description;
        Ok(Box::new(table))
    }
}
