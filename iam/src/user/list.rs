use tokio_stream::StreamExt;

use super::*;

#[derive(Debug, Args)]
pub struct ListUsers {}

impl ListUsers {
    pub async fn execute(self, client: iam::Client) -> IamResult<Vec<User>> {
        let users = client
            .list_users()
            .into_paginator()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?
            .into_iter()
            .filter_map(|output| output.users)
            .flatten()
            .collect();
        Ok(users)
    }
}
