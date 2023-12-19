use super::*;

/// Retrieves information about IAM entity usage and IAM quotas in the Amazon Web Services account.
#[derive(Debug, Args)]
pub struct GetAccountSummary;

#[async_trait]
impl Execute for GetAccountSummary {
    async fn execute(self: Box<Self>, config: &Config) -> IamResult {
        let summary = config
            .client()
            .get_account_summary()
            .send()
            .await?
            .summary_map
            .unwrap_or_default();
        Ok(Box::new(summary))
    }
}
