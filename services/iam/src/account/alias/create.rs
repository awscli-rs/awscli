use super::*;

/// Creates an alias for your Amazon Web Services account.
#[derive(Debug, Args)]
pub struct CreateAccountAlias {
    /// The account alias to create.
    #[arg(long)]
    account_alias: String,
}

impl CreateAccountAlias {
    pub(crate) async fn execute(self, config: &Config) -> IamResult {
        config
            .iam()
            .create_account_alias()
            .account_alias(self.account_alias)
            .send()
            .await?;
        Ok(Box::new(()))
    }
}
