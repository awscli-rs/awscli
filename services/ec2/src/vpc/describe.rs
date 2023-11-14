use super::*;

/// Describes one or more of your VPCs.
#[derive(Debug, Args)]
pub struct DescribeVpcs {
    /// The IDs of the VPCs.
    #[arg(long)]
    vpc_ids: Option<Vec<String>>,

    /// Checks whether you have the required permissions for the action,
    /// without actually making the request, and provides an error response.
    #[arg(long)]
    dry_run: Option<bool>,
}

#[async_trait]
impl Execute for DescribeVpcs {
    async fn execute(self: Box<Self>, config: &Config) -> Ec2Result {
        let vpcs = config
            .client()
            .describe_vpcs()
            .set_vpc_ids(self.vpc_ids)
            .set_dry_run(self.dry_run)
            .send()
            .await?
            .vpcs;

        Ok(Box::new(vpcs))
    }
}
