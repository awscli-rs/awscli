use async_trait::async_trait;
use aws_sdk_account::types;
use aws_sdk_account::Client;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod region;

type AccountResult<T = Box<dyn show::Show>> = std::result::Result<T, aws_sdk_account::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: Client) -> AccountResult;
}

/// Operations for Amazon Web Services Account Management
#[derive(Debug, Subcommand)]
pub enum Account {
    EnableRegion(region::EnableRegion),
    DisableRegion(region::DisableRegion),
    ListRegions(region::ListRegions),
    GetRegionOptStatus(region::GetRegionOptStatus),
}

impl Account {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::EnableRegion(enable_region) => Box::new(enable_region),
            Self::DisableRegion(disable_region) => Box::new(disable_region),
            Self::ListRegions(list_regions) => Box::new(list_regions),
            Self::GetRegionOptStatus(status) => Box::new(status),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<aws_sdk_account::Error>> {
        let client = Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
