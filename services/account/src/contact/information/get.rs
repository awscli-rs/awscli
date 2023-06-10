use super::*;

/// Retrieves the primary contact information of an Amazon Web Services account.
#[derive(Debug, Args)]
pub struct GetContactInformation {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,
}

#[async_trait]
impl Execute for GetContactInformation {
    async fn execute(self: Box<Self>, client: Client) -> AccountResult {
        let contact = client
            .get_contact_information()
            .set_account_id(self.account_id)
            .send()
            .await?
            .contact_information;

        Ok(Box::new(contact))
    }
}
