use async_trait::async_trait;
use aws_sdk_ec2 as ec2;
use clap::{Args, Subcommand};
use ec2::types;

use config::Config;
use error::RawsError;

mod vpc;

type Ec2Result<T = Box<dyn show::Show>> = std::result::Result<T, ec2::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, config: &Config) -> Ec2Result;
}

trait ClientExt {
    fn client(&self) -> ec2::Client;
}

impl ClientExt for Config {
    fn client(&self) -> ec2::Client {
        ec2::Client::new(self.config())
    }
}

/// Amazon Elastic Compute Cloud (Amazon EC2) provides secure and resizable
/// computing capacity in the Amazon Web Services Cloud.
#[derive(Debug, Subcommand)]
#[allow(clippy::large_enum_variant)]
pub enum Ec2 {
    CreateVpc(vpc::CreateVpc),
    DeleteVpc(vpc::DeleteVpc),
    DescribeVpcs(vpc::DescribeVpcs),
}

impl Ec2 {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::CreateVpc(create_vpc) => Box::new(create_vpc),
            Self::DeleteVpc(delete_vpc) => Box::new(delete_vpc),
            Self::DescribeVpcs(describe_vpcs) => Box::new(describe_vpcs),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<ec2::Error>> {
        self.boxed()
            .execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
