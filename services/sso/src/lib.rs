use async_trait::async_trait;
use aws_sdk_sso as sso;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod account;

type SsoResult<T = Box<dyn show::Show>> = std::result::Result<T, sso::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: sso::Client) -> SsoResult;
}

/// AWS IAM Identity Center (successor to AWS Single Sign-On)
#[derive(Debug, Subcommand)]
pub enum Sso {
    ListAccounts(account::ListAccounts),
}

impl Sso {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::ListAccounts(list_accounts) => Box::new(list_accounts),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<sso::Error>> {
        let client = sso::Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
