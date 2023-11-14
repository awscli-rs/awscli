use super::*;

/// Returns descriptive information about an Amazon EKS cluster.
#[derive(Debug, Args)]
pub struct DescribeCluster {
    /// The name of the cluster to describe.
    #[arg(long)]
    name: String,
}

#[async_trait]
impl Execute for DescribeCluster {
    async fn execute(self: Box<Self>, config: &Config) -> EksResult {
        let cluster = config
            .client()
            .describe_cluster()
            .name(self.name)
            .send()
            .await?
            .cluster;
        Ok(Box::new(cluster))
    }
}
