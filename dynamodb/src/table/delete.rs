use super::*;

#[derive(Debug, Args)]
pub struct DeleteTable {
    /// The name of the table to delete.
    #[arg(long)]
    table_name: String,
}

impl DeleteTable {
    pub async fn execute(self, client: dynamo::Client) -> DynamoResult<Option<TableDescription>> {
        let table = client
            .delete_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table_description;
        Ok(table)
    }
}
