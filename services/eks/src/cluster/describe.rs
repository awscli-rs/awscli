use super::*;

/// Returns descriptive information about an Amazon EKS cluster.
#[derive(Debug, Args)]
pub struct DescribeCluster {
    /// The name of the cluster to describe.
    #[arg(long)]
    name: String,
}

impl DescribeCluster {
    pub(crate) async fn execute(self, config: &Config) -> EksResult {
        let cluster = config
            .eks()
            .describe_cluster()
            .name(self.name)
            .send()
            .await?
            .cluster;
        Ok(Box::new(cluster))
    }
}
