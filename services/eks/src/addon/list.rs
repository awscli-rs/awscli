use super::*;

/// Lists the available add-ons.
#[derive(Debug, Args)]
pub struct ListAddons {
    /// The name of the Amazon EKS cluster.
    #[arg(long)]
    cluster_name: String,
}

impl ListAddons {
    pub(crate) async fn execute(self, config: &Config) -> EksResult {
        let addons = config
            .eks()
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
