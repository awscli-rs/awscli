use super::*;

/// Returns a set of temporary security credentials that you can use to
/// access AWS resources
#[derive(Debug, Args)]
pub struct GetCallerIdentity;

#[async_trait]
impl Execute for GetCallerIdentity {
    async fn execute(self: Box<Self>, client: sts::Client) -> StsResult {
        let identity = client.get_caller_identity().send().await?;
        Ok(Box::new(identity))
    }
}
