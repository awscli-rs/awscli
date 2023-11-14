use super::*;

/// Lists the Amazon EKS clusters in your Amazon Web Services account.
#[derive(Debug, Args)]
pub struct ListClusters;

#[async_trait]
impl Execute for ListClusters {
    async fn execute(self: Box<Self>, config: &Config) -> EksResult {
        let cluster = config
            .client()
            .list_clusters()
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(cluster))
    }
}
