use super::*;

/// Lists the available add-ons.
#[derive(Debug, Args)]
pub struct ListAddons {
    /// The name of the Amazon EKS cluster.
    #[arg(long)]
    cluster_name: String,
}

#[async_trait]
impl Execute for ListAddons {
    async fn execute(self: Box<Self>, client: eks::Client) -> EksResult {
        let addons = client
            .list_addons()
            .cluster_name(self.cluster_name)
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(addons))
    }
}
