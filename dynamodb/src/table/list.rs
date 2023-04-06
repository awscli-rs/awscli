use super::*;

#[derive(Debug, Args)]
pub struct ListTables {}

impl ListTables {
    pub async fn execute(self, client: dynamo::Client) -> DynamoResult {
        let output = client.list_tables().send().await?;
        println!("{output:?}");
        Ok(())
    }
}
