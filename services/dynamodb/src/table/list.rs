use super::*;

#[derive(Debug, Args)]
pub struct ListTables {}

#[async_trait]
impl Execute for ListTables {
    async fn execute(self: Box<Self>, config: &Config) -> DynamoResult {
        let tables = config
            .client()
            .list_tables()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(tables))
    }
}
