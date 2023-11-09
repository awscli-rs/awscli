use super::*;

/// Retrieves the specified alternate contact attached to an Amazon Web Services account.
#[derive(Debug, Args)]
pub struct GetAlternateContact {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,

    /// Specifies which alternate contact you want to retrieve.
    #[arg(long, value_parser = clap::value_parser!(account::types::AlternateContactType))]
    alternate_contact_type: account::types::AlternateContactType,
}

#[async_trait]
impl Execute for GetAlternateContact {
    async fn execute(self: Box<Self>, config: &Config) -> AccountResult {
        let contact = Self::client(config)
            .get_alternate_contact()
            .set_account_id(self.account_id)
            .alternate_contact_type(self.alternate_contact_type)
            .send()
            .await?
            .alternate_contact;

        Ok(Box::new(contact))
    }
}
