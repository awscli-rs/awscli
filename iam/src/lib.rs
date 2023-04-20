use std::error::Error;

use aws_sdk_iam as iam;
use clap::{Args, Subcommand};

use config::Config;
use error::{AwsError, RawsError};
use iam::error::ProvideErrorMetadata;

type IamResult<T = Box<dyn show::Show>> = std::result::Result<T, iam::Error>;

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
        let r = match self {
            Self::CreateUser(create_user) => create_user.execute(client).await,
            //.map(Box::new),
            // .map(|output| config.output(output))?
            Self::DeleteUser(delete_user) => delete_user.execute(client).await,
            //s.map(Box::new),
            // .map(|output| config.output(output))?
            Self::GetUser(get_user) => get_user.execute(client).await,
            // .map(Box::new),
            // .map(|output| config.output(output))?
            Self::ListUsers(list_users) => list_users.execute(client).await,
            // .map(Box::new),
            // .map(|output| config.show(output))?,
        };

        r.map(|item| config.show(item)).map_err(Into::into)

        // .map_err(|e| {
        //     match e.try_into_unhandled() {
        //         Ok(unhandled) => {
        //             unhandled.into()

        //             //     if let Some(source) = unhandled.source() {
        //             //         source.into()
        //             //     } else {
        //             //         iam::Error::Unhandled(unhandled).into()
        //             //     }
        //         }

        //         Err(other) => other.into(),
        //     }
        //     println!("Source: {:?}", e.source());
        //     if let Some(source) = e.source() {
        //         e.message()
        //         println!("Source of source: {:?} | {}", source, source.);
        //     }
        //     println!("{:?}", e.message());
        // }

        // println!("{:?}", e.error_context());
        // })
    }
}
