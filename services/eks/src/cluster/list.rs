use super::*;

/// Lists the Amazon EKS clusters in your Amazon Web Services account.
#[derive(Debug, Args)]
pub struct ListClusters;

impl ListClusters {
    pub(crate) async fn execute(self, config: &Config) -> EksResult {
        let cluster = config
            .eks()
            .list_clusters()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(cluster))
    }
}
