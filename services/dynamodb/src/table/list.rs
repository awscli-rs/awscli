use super::*;

#[derive(Debug, Args)]
pub struct ListTables {}

#[async_trait]
impl Execute for ListTables {
    async fn execute(self: Box<Self>, client: Client) -> DynamoResult {
        let tables = client
            .list_tables()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(tables))
    }
}
