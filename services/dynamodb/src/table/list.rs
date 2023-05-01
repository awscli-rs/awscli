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
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?
            .into_iter()
            .filter_map(|output| output.table_names)
            .flatten()
            .collect::<Vec<_>>();
        Ok(Box::new(tables))
    }
}
