use super::*;

/// Deletes the specified Amazon Web Services account alias.
#[derive(Debug, Args)]
pub struct DeleteAccountAlias {
    /// The name of the account alias to delete.
    #[arg(long)]
    account_alias: String,
}

impl DeleteAccountAlias {
    pub(crate) async fn execute(self, config: &Config) -> IamResult {
        config
            .iam()
            .delete_account_alias()
            .account_alias(self.account_alias)
            .send()
            .await?;
        Ok(Box::new(()))
    }
}
