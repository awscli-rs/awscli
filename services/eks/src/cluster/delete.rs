use super::*;

/// Deletes the Amazon EKS cluster control plane.
#[derive(Debug, Args)]
pub struct DeleteCluster {
    /// The name of the cluster to delete.
    #[arg(long)]
    name: String,
}

impl DeleteCluster {
    pub(crate) async fn execute(self, config: &Config) -> EksResult {
        let cluster = config
            .eks()
            .delete_cluster()
            .name(self.name)
            .send()
            .await?
            .cluster;
        Ok(Box::new(cluster))
    }
}
