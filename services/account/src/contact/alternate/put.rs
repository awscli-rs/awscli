use super::*;

/// Modifies the specified alternate contact attached to an Amazon Web Services account.
#[derive(Debug, Args)]
pub struct PutAlternateContact {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,

    /// Specifies which alternate contact you want to retrieve.
    #[arg(long, value_parser = clap::value_parser!(account::types::AlternateContactType))]
    alternate_contact_type: account::types::AlternateContactType,

    /// Specifies a name for the alternate contact.
    #[arg(long)]
    name: String,

    /// Specifies a title for the alternate contact.
    #[arg(long)]
    title: String,

    /// Specifies an email address for the alternate contact.
    #[arg(long)]
    email_address: String,

    /// Specifies a phone number for the alternate contact.
    #[arg(long)]
    phone_number: String,
}

#[async_trait]
impl Execute for PutAlternateContact {
    async fn execute(self: Box<Self>, config: &Config) -> AccountResult {
        let _output = Self::client(config)
            .put_alternate_contact()
            .set_account_id(self.account_id)
            .alternate_contact_type(self.alternate_contact_type)
            .name(self.name)
            .title(self.title)
            .email_address(self.email_address)
            .phone_number(self.phone_number)
            .send()
            .await?;

        Ok(Box::new(()))
    }
}
