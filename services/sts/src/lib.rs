use async_trait::async_trait;
use aws_sdk_sts as sts;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod get;
mod role;

type StsResult<T = Box<dyn show::Show>> = std::result::Result<T, sts::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: sts::Client) -> StsResult;
}

/// Security Token Service (STS) operations
#[derive(Debug, Subcommand)]
pub enum Sts {
    AssumeRole(role::AssumeRole),
    GetCallerIdentity(get::GetCallerIdentity),
    GetAccessKeyInfo(get::GetAccessKeyInfo),
}

impl Sts {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::AssumeRole(assume_role) => Box::new(assume_role),
            Self::GetCallerIdentity(get_caller_identity) => Box::new(get_caller_identity),
            Self::GetAccessKeyInfo(get_access_key_info) => Box::new(get_access_key_info),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<sts::Error>> {
        let client = sts::Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
