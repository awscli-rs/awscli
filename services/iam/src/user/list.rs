use super::*;

#[derive(Debug, Args)]
pub struct ListUsers {}

#[async_trait]
impl Execute for ListUsers {
    async fn execute(self: Box<Self>, client: iam::Client) -> IamResult {
        let users = client
            .list_users()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;
        Ok(Box::new(users))
    }
}
