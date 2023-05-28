use async_trait::async_trait;
use aws_sdk_eks as eks;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod cluster;

type EksResult<T = Box<dyn show::Show>> = std::result::Result<T, eks::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: eks::Client) -> EksResult;
}

#[derive(Debug, Subcommand)]
pub enum Eks {
    CreateCluster(cluster::CreateCluster),
    DeleteCluster(cluster::DeleteCluster),
    ListClusters(cluster::ListClusters),
    DescribeCluster(cluster::DescribeCluster),
}

impl Eks {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::CreateCluster(create_cluster) => Box::new(create_cluster),
            Self::DeleteCluster(delete_cluster) => Box::new(delete_cluster),
            Self::ListClusters(list_cluster) => Box::new(list_cluster),
            Self::DescribeCluster(describe_cluster) => Box::new(describe_cluster),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<eks::Error>> {
        let client = eks::Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
