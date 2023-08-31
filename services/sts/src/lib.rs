use async_trait::async_trait;
use aws_sdk_sts as sts;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod assume;
mod get;

type StsResult<T = Box<dyn show::Show>> = std::result::Result<T, sts::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: sts::Client) -> StsResult;
}

/// Security Token Service (STS) operations
#[derive(Debug, Subcommand)]
pub enum Sts {
    AssumeRole(assume::AssumeRole),
    AssumeRoleWithSaml(assume::AssumeRoleWithSaml),
    AssumeRoleWithWebIdentity,
    GetAccessKeyInfo(get::GetAccessKeyInfo),
    GetCallerIdentity(get::GetCallerIdentity),
    GetFederationToken,
    GetSessionToken(get::GetSessionToken),
}

impl Sts {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::AssumeRole(assume_role) => Box::new(assume_role),
            Self::AssumeRoleWithSaml(assume_role_with_saml) => Box::new(assume_role_with_saml),
            Self::AssumeRoleWithWebIdentity => todo!(),
            Self::GetAccessKeyInfo(get_access_key_info) => Box::new(get_access_key_info),
            Self::GetCallerIdentity(get_caller_identity) => Box::new(get_caller_identity),
            Self::GetFederationToken => todo!(),
            Self::GetSessionToken(get_session_token) => Box::new(get_session_token),
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
