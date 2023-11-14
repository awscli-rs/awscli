use super::*;

#[derive(Debug, Args)]
pub struct ListUsers {}

#[async_trait]
impl Execute for ListUsers {
    async fn execute(self: Box<Self>, config: &Config) -> IamResult {
        let users = config
            .client()
            .list_users()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;
        Ok(Box::new(users))
    }
}
