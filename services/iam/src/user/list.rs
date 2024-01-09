use super::*;

#[derive(Debug, Args)]
pub struct ListUsers {}

impl ListUsers {
    pub(crate) async fn execute(self, config: &Config) -> IamResult {
        let users = config
            .iam()
            .list_users()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;
        Ok(Box::new(users))
    }
}
