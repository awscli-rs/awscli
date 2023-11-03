use super::*;

/// Lists the Amazon EKS clusters in your Amazon Web Services account.
#[derive(Debug, Args)]
pub struct ListUpdates {
    /// The name of the Amazon EKS cluster to list updates for.
    #[arg(long)]
    name: String,

    /// The name of the Amazon EKS managed node group to list updates for.
    #[arg(long)]
    nodegroup_name: Option<String>,

    /// The names of the installed add-ons that have available updates.
    #[arg(long)]
    addon_name: Option<String>,
}

#[async_trait]
impl Execute for ListUpdates {
    async fn execute(self: Box<Self>, client: eks::Client) -> EksResult {
        let updates = client
            .list_updates()
            .name(self.name)
            .set_nodegroup_name(self.nodegroup_name)
            .set_addon_name(self.addon_name)
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(updates))
    }
}
