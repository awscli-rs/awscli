use super::*;

#[derive(Debug, Args)]
pub struct ListTables {}

impl ListTables {
    pub(crate) async fn execute(self, config: &Config) -> DynamoResult {
        let tables = config
            .dynamodb()
            .list_tables()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(tables))
    }
}
