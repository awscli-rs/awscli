use aws_sdk_eks as eks;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod addon;
mod cluster;
mod update;

type EksResult<T = Box<dyn show::Show>> = std::result::Result<T, eks::Error>;

/// Amazon Elastic Kubernetes Service (Amazon EKS)
#[derive(Debug, Subcommand)]
pub enum Eks {
    CreateCluster(cluster::CreateCluster),
    DeleteCluster(cluster::DeleteCluster),
    ListClusters(cluster::ListClusters),
    DescribeCluster(cluster::DescribeCluster),
    ListUpdates(update::ListUpdates),
    DescribeUpdate(update::DescribeUpdate),
    ListAddons(addon::ListAddons),
    DescribeAddon(addon::DescribeAddon),
}

impl Eks {
    async fn execute(self, config: &Config) -> EksResult {
        match self {
            Self::CreateCluster(create_cluster) => create_cluster.execute(config).await,
            Self::DeleteCluster(delete_cluster) => delete_cluster.execute(config).await,
            Self::ListClusters(list_cluster) => list_cluster.execute(config).await,
            Self::DescribeCluster(describe_cluster) => describe_cluster.execute(config).await,
            Self::ListUpdates(list_updates) => list_updates.execute(config).await,
            Self::DescribeUpdate(describe_update) => describe_update.execute(config).await,
            Self::ListAddons(list_addons) => list_addons.execute(config).await,
            Self::DescribeAddon(describe_addon) => describe_addon.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<eks::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
