use super::*;

/// Enables (opts-in) a particular Region for an account.
#[derive(Debug, Args)]
pub struct EnableRegion {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,

    /// Specifies the Region-code for a given Region name
    #[arg(long)]
    region_name: String,
}

#[async_trait]
impl Execute for EnableRegion {
    async fn execute(self: Box<Self>, client: Client) -> AccountResult {
        let _output = client
            .enable_region()
            .set_account_id(self.account_id)
            .region_name(self.region_name)
            .send()
            .await?;

        Ok(Box::new(()))
    }
}
