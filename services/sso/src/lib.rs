use async_trait::async_trait;
use aws_sdk_sso as sso;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod account;
mod role;

type SsoResult<T = Box<dyn show::Show>> = std::result::Result<T, sso::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: sso::Client) -> SsoResult;
}

/// AWS IAM Identity Center (successor to AWS Single Sign-On)
#[derive(Debug, Subcommand)]
pub enum Sso {
    ListAccounts(account::ListAccounts),
    ListAccountRoles(account::ListAccountRoles),
    GetRoleCredentials(role::GetRoleCredentials),
    Login(account::Login),
    Logout(account::Logout),
}

impl Sso {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::ListAccounts(list_accounts) => Box::new(list_accounts),
            Self::ListAccountRoles(list_account_roles) => Box::new(list_account_roles),
            Self::GetRoleCredentials(get_role_credentials) => Box::new(get_role_credentials),
            Self::Login(login) => Box::new(login),
            Self::Logout(logout) => Box::new(logout),
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
