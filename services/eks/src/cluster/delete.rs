use super::*;

/// Deletes the Amazon EKS cluster control plane.
#[derive(Debug, Args)]
pub struct DeleteCluster {
    /// The name of the cluster to delete.
    #[arg(long)]
    name: String,
}

#[async_trait]
impl Execute for DeleteCluster {
    async fn execute(self: Box<Self>, config: &Config) -> EksResult {
        let cluster = config
            .client()
            .delete_cluster()
            .name(self.name)
            .send()
            .await?
            .cluster;
        Ok(Box::new(cluster))
    }
}
