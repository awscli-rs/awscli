use super::*;

/// Deletes the specified VPC.
#[derive(Debug, Args)]
pub struct DeleteVpc {
    /// The ID of the VPC.
    #[arg(long)]
    vpc_id: String,

    /// Checks whether you have the required permissions for the action,
    /// without actually making the request, and provides an error response.
    #[arg(long)]
    dry_run: Option<bool>,
}

#[async_trait]
impl Execute for DeleteVpc {
    async fn execute(self: Box<Self>, client: ec2::Client) -> Ec2Result {
        client
            .delete_vpc()
            .vpc_id(self.vpc_id)
            .set_dry_run(self.dry_run)
            .send()
            .await?;

        Ok(Box::new(()))
    }
}
