use aws_sdk_iam as iam;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod account;
mod user;

type IamResult<T = Box<dyn show::Show>> = std::result::Result<T, iam::Error>;

/// Identity and Access Management (IAM)
#[derive(Debug, Subcommand)]
pub enum Iam {
    CreateUser(user::CreateUser),
    DeleteUser(user::DeleteUser),
    GetUser(user::GetUser),
    ListUsers(user::ListUsers),
    GetAccountSummary(account::GetAccountSummary),
    GetAccountAuthorizationDetails(account::GetAccountAuthorizationDetails),
    CreateAccountAlias(account::CreateAccountAlias),
    DeleteAccountAlias(account::DeleteAccountAlias),
    ListAccountAliases(account::ListAccountAliases),
}

impl Iam {
    async fn execute(self, config: &Config) -> IamResult {
        match self {
            Self::CreateUser(create_user) => create_user.execute(config).await,
            Self::DeleteUser(delete_user) => delete_user.execute(config).await,
            Self::GetUser(get_user) => get_user.execute(config).await,
            Self::ListUsers(list_users) => list_users.execute(config).await,
            Self::GetAccountSummary(account_summary) => account_summary.execute(config).await,
            Self::GetAccountAuthorizationDetails(authz_details) => {
                authz_details.execute(config).await
            }
            Self::CreateAccountAlias(create_alias) => create_alias.execute(config).await,
            Self::DeleteAccountAlias(delete_alias) => delete_alias.execute(config).await,
            Self::ListAccountAliases(list_aliases) => list_aliases.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<iam::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
