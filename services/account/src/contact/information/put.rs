use super::*;

/// Updates the primary contact information of an Amazon Web Services account.
#[derive(Debug, Args)]
pub struct PutContactInformation {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,

    /// Contains the details of the primary contact information associated with an Amazon Web Services account.
    #[arg(long, value_parser = parsers::account::contact_information)]
    contact_information: account::types::ContactInformation,
}

impl PutContactInformation {
    pub(crate) async fn execute(self, config: &Config) -> AccountResult {
        let contact = config
            .account()
            .get_alternate_contact()
            .set_account_id(self.account_id)
            .send()
            .await?
            .alternate_contact;

        Ok(Box::new(contact))
    }
}
