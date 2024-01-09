use super::*;

/// Returns the account identifier for the specified access key ID.
#[derive(Debug, Args)]
pub struct GetAccessKeyInfo {
    /// The identifier of an access key.
    #[arg(long)]
    access_key_id: String,
}

impl GetAccessKeyInfo {
    pub(crate) async fn execute(self, config: &Config) -> StsResult {
        let output = config
            .sts()
            .get_access_key_info()
            .access_key_id(self.access_key_id)
            .send()
            .await?;
        Ok(Box::new(output))
    }
}
