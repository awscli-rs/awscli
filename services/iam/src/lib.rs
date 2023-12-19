use async_trait::async_trait;
use aws_sdk_iam as iam;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod account;
mod user;

type IamResult<T = Box<dyn show::Show>> = std::result::Result<T, iam::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, config: &Config) -> IamResult;
}

trait ClientExt {
    fn client(&self) -> iam::Client;
}

impl ClientExt for Config {
    fn client(&self) -> iam::Client {
        iam::Client::new(self.config())
    }
}

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
    ListAccountAliases(account::ListAccountAliases),
}

impl Iam {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::CreateUser(create_user) => Box::new(create_user),
            Self::DeleteUser(delete_user) => Box::new(delete_user),
            Self::GetUser(get_user) => Box::new(get_user),
            Self::ListUsers(list_users) => Box::new(list_users),
            Self::GetAccountSummary(account_summary) => Box::new(account_summary),
            Self::GetAccountAuthorizationDetails(authz_details) => Box::new(authz_details),
            Self::CreateAccountAlias(create_alias) => Box::new(create_alias),
            Self::ListAccountAliases(list_aliases) => Box::new(list_aliases),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<iam::Error>> {
        self.boxed()
            .execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
