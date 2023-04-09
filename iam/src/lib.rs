use aws_sdk_iam as iam;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

type IamResult<T> = std::result::Result<T, iam::Error>;

mod user;

#[derive(Debug, Subcommand)]
pub enum Iam {
    CreateUser(user::CreateUser),
    DeleteUser(user::DeleteUser),
    GetUser(user::GetUser),
    ListUsers(user::ListUsers),
}

impl Iam {
    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<iam::Error>> {
        let client = iam::Client::new(config.config());
        match self {
            Self::CreateUser(create_user) => create_user
                .execute(client)
                .await
                .map(|output| config.output(output))?,
            Self::DeleteUser(delete_user) => delete_user
                .execute(client)
                .await
                .map(|output| config.output(output))?,
            Self::GetUser(get_user) => get_user
                .execute(client)
                .await
                .map(|output| config.output(output))?,
            Self::ListUsers(list_users) => list_users
                .execute(client)
                .await
                .map(|output| config.output(output))?,
        }
        Ok(())
    }
}
