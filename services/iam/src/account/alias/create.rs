use super::*;

/// Creates an alias for your Amazon Web Services account.
#[derive(Debug, Args)]
pub struct CreateAccountAlias {
    /// The account alias to create.
    #[arg(long)]
    account_alias: String,
}

#[async_trait]
impl Execute for CreateAccountAlias {
    async fn execute(self: Box<Self>, config: &Config) -> IamResult {
        config
            .client()
            .create_account_alias()
            .account_alias(self.account_alias)
            .send()
            .await?;
        Ok(Box::new(()))
    }
}
