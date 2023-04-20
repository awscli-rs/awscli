use tokio_stream::StreamExt;

use super::*;

#[derive(Debug, Args)]
pub struct ListUsers {}

#[async_trait]
impl Execute for ListUsers {
    async fn execute(self: Box<Self>, client: iam::Client) -> IamResult {
        let users = client
            .list_users()
            .into_paginator()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?
            .into_iter()
            .filter_map(|output| output.users)
            .flatten()
            .collect::<Vec<_>>();
        Ok(Box::new(users))
    }
}
