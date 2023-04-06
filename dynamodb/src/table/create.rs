use super::*;

#[derive(Debug, Args)]
pub struct CreateTable {
    /// The name of the table to create.
    #[arg(long)]
    table_name: String,
}

impl CreateTable {
    pub async fn execute(self, client: dynamo::Client) -> DynamoResult<Option<TableDescription>> {
        let table = client
            .create_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table_description;
        Ok(table)
    }
}
