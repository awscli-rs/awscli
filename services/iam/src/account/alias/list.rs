use super::*;

/// Lists the account alias associated with the Amazon Web Services account
/// (Note: you can have only one).
#[derive(Debug, Args)]
pub struct ListAccountAliases;

impl ListAccountAliases {
    pub(crate) async fn execute(self, config: &Config) -> IamResult {
        let aliases = config
            .iam()
            .list_account_aliases()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;
        Ok(Box::new(aliases))
    }
}
