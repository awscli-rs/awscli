use tokio_stream::StreamExt;

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
            .collect::<Result<Vec<_>, _>>()
            .await?;

        Ok(Box::new(tables))
    }
}
