use super::*;

/// Lists the account alias associated with the Amazon Web Services account
/// (Note: you can have only one).
#[derive(Debug, Args)]
pub struct ListAccountAliases;

#[async_trait]
impl Execute for ListAccountAliases {
    async fn execute(self: Box<Self>, config: &Config) -> IamResult {
        let aliases = config
            .client()
            .list_account_aliases()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;
        Ok(Box::new(aliases))
    }
}
