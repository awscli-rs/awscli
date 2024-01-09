use super::*;

/// Retrieves information about IAM entity usage and IAM quotas in the Amazon Web Services account.
#[derive(Debug, Args)]
pub struct GetAccountSummary;

impl GetAccountSummary {
    pub(crate) async fn execute(self, config: &Config) -> IamResult {
        let summary = config
            .iam()
            .get_account_summary()
            .send()
            .await?
            .summary_map
            .unwrap_or_default();
        Ok(Box::new(summary))
    }
}
