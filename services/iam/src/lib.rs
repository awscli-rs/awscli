use async_trait::async_trait;
use aws_sdk_iam as iam;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod user;

type IamResult<T = Box<dyn show::Show>> = std::result::Result<T, iam::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: iam::Client) -> IamResult;
}

/// Identity and Access Management (IAM)
#[derive(Debug, Subcommand)]
pub enum Iam {
    CreateUser(user::CreateUser),
    DeleteUser(user::DeleteUser),
    GetUser(user::GetUser),
    ListUsers(user::ListUsers),
}

impl Iam {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::CreateUser(create_user) => Box::new(create_user),
            Self::DeleteUser(delete_user) => Box::new(delete_user),
            Self::GetUser(get_user) => Box::new(get_user),
            Self::ListUsers(list_users) => Box::new(list_users),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<iam::Error>> {
        let client = iam::Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
