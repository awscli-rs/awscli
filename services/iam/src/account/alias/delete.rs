use super::*;

/// Deletes the specified Amazon Web Services account alias.
#[derive(Debug, Args)]
pub struct DeleteAccountAlias {
    /// The name of the account alias to delete.
    #[arg(long)]
    account_alias: String,
}

#[async_trait]
impl Execute for DeleteAccountAlias {
    async fn execute(self: Box<Self>, config: &Config) -> IamResult {
        config
            .client()
            .delete_account_alias()
            .account_alias(self.account_alias)
            .send()
            .await?;
        Ok(Box::new(()))
    }
}
