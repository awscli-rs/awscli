use super::*;

#[derive(Debug, Args)]
pub struct DeleteTable {
    /// The name of the table to delete.
    #[arg(long)]
    table_name: String,
}

impl DeleteTable {
    pub(crate) async fn execute(self, config: &Config) -> DynamoResult {
        let table = config
            .dynamodb()
            .delete_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table_description;
        Ok(Box::new(table))
    }
}
