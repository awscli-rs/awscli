use tokio_stream::StreamExt;

use super::*;

#[derive(Debug, Args)]
pub struct Scan {
    /// The name of the table to scan.
    #[arg(long)]
    table_name: String,
}

#[async_trait]
impl Execute for Scan {
    async fn execute(self: Box<Self>, client: dynamodb::Client) -> DynamoResult {
        let tables = client
            .scan()
            .table_name(self.table_name)
            .into_paginator()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?
            .into_iter()
            .filter_map(|output| output.items)
            .flatten()
            .collect::<Vec<_>>();
        Ok(Box::new(tables))
    }
}
