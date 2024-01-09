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

impl DeleteVpc {
    pub(crate) async fn execute(self, config: &Config) -> Ec2Result {
        config
            .ec2()
            .delete_vpc()
            .vpc_id(self.vpc_id)
            .set_dry_run(self.dry_run)
            .send()
            .await?;

        Ok(Box::new(()))
    }
}
