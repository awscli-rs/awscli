use super::*;

/// Describes an Amazon EKS add-on.
#[derive(Debug, Args)]
pub struct DescribeAddon {
    /// The name of the Amazon EKS cluster.
    #[arg(long)]
    cluster_name: String,

    /// The name of the add-on. The name must match one of the names returned by `list-addons`
    #[arg(long)]
    addon_name: String,
}

#[async_trait]
impl Execute for DescribeAddon {
    async fn execute(self: Box<Self>, config: &Config) -> EksResult {
        let addon = config
            .client()
            .describe_addon()
            .cluster_name(self.cluster_name)
            .addon_name(self.addon_name)
            .send()
            .await?
            .addon;
        Ok(Box::new(addon))
    }
}
