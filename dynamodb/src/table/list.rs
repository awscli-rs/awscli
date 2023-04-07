use tokio_stream::StreamExt;

use super::*;

#[derive(Debug, Args)]
pub struct ListTables {}

impl ListTables {
    pub async fn execute(self, client: dynamodb::Client) -> DynamoResult<Vec<String>> {
        let tables = client
            .list_tables()
            .into_paginator()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?
            .into_iter()
            .filter_map(|output| output.table_names)
            .flatten()
            .collect();
        Ok(tables)
    }
}
