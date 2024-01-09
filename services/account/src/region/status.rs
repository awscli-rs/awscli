use super::*;

/// Retrieves the opt-in status of a particular Region.
#[derive(Debug, Args)]
pub struct GetRegionOptStatus {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,

    /// Specifies the Region-code for a given Region name
    #[arg(long)]
    region_name: String,
}

impl GetRegionOptStatus {
    pub(crate) async fn execute(self, config: &Config) -> AccountResult {
        let status = config
            .account()
            .get_region_opt_status()
            .set_account_id(self.account_id)
            .region_name(self.region_name)
            .send()
            .await?;

        Ok(Box::new(status))
    }
}
