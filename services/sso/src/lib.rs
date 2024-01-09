use aws_sdk_sso as sso;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod account;
mod role;

type SsoResult<T = Box<dyn show::Show>> = std::result::Result<T, sso::Error>;

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
    async fn execute(self, config: &Config) -> SsoResult {
        match self {
            Self::ListAccounts(list_accounts) => list_accounts.execute(config).await,
            Self::ListAccountRoles(list_account_roles) => list_account_roles.execute(config).await,
            Self::GetRoleCredentials(get_role_credentials) => {
                get_role_credentials.execute(config).await
            }
            Self::Login(login) => login.execute(config).await,
            Self::Logout(logout) => logout.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<sso::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
