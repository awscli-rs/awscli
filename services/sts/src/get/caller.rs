use super::*;

/// Returns details about the IAM user or role whose credentials are used
/// to call the operation.
#[derive(Debug, Args)]
pub struct GetCallerIdentity;

impl GetCallerIdentity {
    pub(crate) async fn execute(self, config: &Config) -> StsResult {
        let identity = config.sts().get_caller_identity().send().await?;
        Ok(Box::new(identity))
    }
}
