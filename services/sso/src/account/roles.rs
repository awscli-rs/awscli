use tokio_stream::StreamExt;

use super::*;

/// Lists all roles that are assigned to the user for a given AWS account.
#[derive(Debug, Args)]
pub struct ListAccountRoles {
    /// The token issued by the CreateToken API call.
    #[arg(long)]
    access_token: String,
    /// The identifier for the AWS account that is assigned to the user.
    #[arg(long)]
    account_id: String,
}

#[async_trait]
impl Execute for ListAccountRoles {
    async fn execute(self: Box<Self>, client: sso::Client) -> SsoResult {
        let account_roles = client
            .list_account_roles()
            .access_token(self.access_token)
            .account_id(self.account_id)
            .into_paginator()
            .items()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?;

        Ok(Box::new(account_roles))
    }
}
