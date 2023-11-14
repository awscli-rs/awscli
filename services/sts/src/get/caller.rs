use super::*;

/// Returns details about the IAM user or role whose credentials are used
/// to call the operation.
#[derive(Debug, Args)]
pub struct GetCallerIdentity;

#[async_trait]
impl Execute for GetCallerIdentity {
    async fn execute(self: Box<Self>, config: &Config) -> StsResult {
        let identity = config.client().get_caller_identity().send().await?;
        Ok(Box::new(identity))
    }
}
