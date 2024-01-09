use super::*;

/// Returns descriptive information about an Amazon EKS cluster.
#[derive(Debug, Args)]
pub struct DescribeUpdate {
    /// The name of the Amazon EKS cluster associated with the update.
    #[arg(long)]
    name: String,

    /// The ID of the update to describe.
    #[arg(long)]
    update_id: String,

    /// The name of the Amazon EKS node group associated with the update. This parameter is required if the update is a node group update.
    #[arg(long)]
    nodegroup_name: Option<String>,

    /// The name of the add-on. The name must match one of the names returned by `list-addons`
    #[arg(long)]
    addon_name: Option<String>,
}

impl DescribeUpdate {
    pub(crate) async fn execute(self, config: &Config) -> EksResult {
        let update = config
            .eks()
            .describe_update()
            .name(self.name)
            .update_id(self.update_id)
            .set_nodegroup_name(self.nodegroup_name)
            .set_addon_name(self.addon_name)
            .send()
            .await?
            .update;
        Ok(Box::new(update))
    }
}
