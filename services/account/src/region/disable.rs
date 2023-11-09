use super::*;

/// Disables (opts-out) a particular Region for an account.
#[derive(Debug, Args)]
pub struct DisableRegion {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,

    /// Specifies the Region-code for a given Region name
    #[arg(long)]
    region_name: String,
}

#[async_trait]
impl Execute for DisableRegion {
    async fn execute(self: Box<Self>, config: &Config) -> AccountResult {
        let _output = Self::client(config)
            .disable_region()
            .set_account_id(self.account_id)
            .region_name(self.region_name)
            .send()
            .await?;

        Ok(Box::new(()))
    }
}
