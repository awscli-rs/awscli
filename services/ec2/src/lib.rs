use aws_sdk_ec2 as ec2;
use clap::{Args, Subcommand};
use ec2::types;

use config::Config;
use error::RawsError;

mod vpc;

type Ec2Result<T = Box<dyn show::Show>> = std::result::Result<T, ec2::Error>;

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
    async fn execute(self, config: &Config) -> Ec2Result {
        match self {
            Self::CreateVpc(create_vpc) => create_vpc.execute(config).await,
            Self::DeleteVpc(delete_vpc) => delete_vpc.execute(config).await,
            Self::DescribeVpcs(describe_vpcs) => describe_vpcs.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<ec2::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
