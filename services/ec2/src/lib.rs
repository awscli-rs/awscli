use async_trait::async_trait;
use aws_sdk_ec2 as ec2;
use clap::{Args, Subcommand};
use ec2::types;
// use tokio_stream::StreamExt;

use config::Config;
use error::RawsError;

mod vpc;

type Ec2Result<T = Box<dyn show::Show>> = std::result::Result<T, ec2::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: ec2::Client) -> Ec2Result;
}

/// Amazon Elastic Compute Cloud (Amazon EC2) provides secure and resizable
/// computing capacity in the Amazon Web Services Cloud.
#[derive(Debug, Subcommand)]
pub enum Ec2 {
    CreateVpc(vpc::CreateVpc),
    DeleteVpc(vpc::DeleteVpc),
}

impl Ec2 {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::CreateVpc(create_vpc) => Box::new(create_vpc),
            Self::DeleteVpc(delete_vpc) => Box::new(delete_vpc),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<ec2::Error>> {
        let client = ec2::Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
