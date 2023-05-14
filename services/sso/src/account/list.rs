use super::*;

/// Lists all AWS accounts assigned to the user.
#[derive(Debug, Args)]
pub struct ListAccounts {
    /// The token issued by the CreateToken API call.
    #[arg(long)]
    access_token: String,
}

#[async_trait]
impl Execute for ListAccounts {
    async fn execute(self: Box<Self>, client: sso::Client) -> SsoResult {
        let list_accounts = client
            .list_accounts()
            .access_token(self.access_token)
            .send()
            .await?
            .account_list
            .unwrap_or_default();

        Ok(Box::new(list_accounts))
    }
}
