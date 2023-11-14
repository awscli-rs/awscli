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
    async fn execute(self: Box<Self>, config: &Config) -> StsResult;
}

trait ClientExt {
    fn client(&self) -> sts::Client;
}

impl ClientExt for Config {
    fn client(&self) -> sts::Client {
        sts::Client::new(self.config())
    }
}

/// Security Token Service (STS) operations
#[derive(Debug, Subcommand)]
pub enum Sts {
    AssumeRole(assume::AssumeRole),
    AssumeRoleWithSaml(assume::AssumeRoleWithSaml),
    AssumeRoleWithWebIdentity(assume::AssumeRoleWithWebIdentity),
    GetAccessKeyInfo(get::GetAccessKeyInfo),
    GetCallerIdentity(get::GetCallerIdentity),
    GetFederationToken(get::GetFederationToken),
    GetSessionToken(get::GetSessionToken),
}

impl Sts {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::AssumeRole(assume_role) => Box::new(assume_role),
            Self::AssumeRoleWithSaml(assume_role_with_saml) => Box::new(assume_role_with_saml),
            Self::AssumeRoleWithWebIdentity(assume_role_with_web_identity) => {
                Box::new(assume_role_with_web_identity)
            }
            Self::GetAccessKeyInfo(get_access_key_info) => Box::new(get_access_key_info),
            Self::GetCallerIdentity(get_caller_identity) => Box::new(get_caller_identity),
            Self::GetFederationToken(get_federation_token) => Box::new(get_federation_token),
            Self::GetSessionToken(get_session_token) => Box::new(get_session_token),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<sts::Error>> {
        self.boxed()
            .execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
