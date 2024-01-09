use super::*;

/// Deletes the specified alternate contact from an Amazon Web Services account.
#[derive(Debug, Args)]
pub struct DeleteAlternateContact {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,

    /// Specifies which alternate contact you want to retrieve.
    #[arg(long, value_parser = clap::value_parser!(account::types::AlternateContactType))]
    alternate_contact_type: account::types::AlternateContactType,
}

impl DeleteAlternateContact {
    pub(crate) async fn execute(self, config: &Config) -> AccountResult {
        let _output = config
            .account()
            .delete_alternate_contact()
            .set_account_id(self.account_id)
            .alternate_contact_type(self.alternate_contact_type)
            .send()
            .await?;

        Ok(Box::new(()))
    }
}
