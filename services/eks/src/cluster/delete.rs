use super::*;

/// Creates an Amazon EKS control plane.
#[derive(Debug, Args)]
pub struct DeleteCluster {
    /// The unique name to give to your cluster.
    #[arg(long)]
    name: String,
}

#[async_trait]
impl Execute for DeleteCluster {
    async fn execute(self: Box<Self>, client: eks::Client) -> EksResult {
        let cluster = client
            .delete_cluster()
            .name(self.name)
            .send()
            .await?
            .cluster;
        Ok(Box::new(cluster))
    }
}
