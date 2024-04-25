use aws_sdk_sts as sts;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod assume;
mod get;

type StsResult<T = Box<dyn show::Show>> = Result<T, sts::Error>;

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
    async fn execute(self, config: &Config) -> StsResult {
        match self {
            Self::AssumeRole(assume_role) => assume_role.execute(config).await,
            Self::AssumeRoleWithSaml(assume_role_with_saml) => {
                assume_role_with_saml.execute(config).await
            }
            Self::AssumeRoleWithWebIdentity(assume_role_with_web_identity) => {
                assume_role_with_web_identity.execute(config).await
            }
            Self::GetAccessKeyInfo(get_access_key_info) => {
                get_access_key_info.execute(config).await
            }
            Self::GetCallerIdentity(get_caller_identity) => {
                get_caller_identity.execute(config).await
            }
            Self::GetFederationToken(get_federation_token) => {
                get_federation_token.execute(config).await
            }
            Self::GetSessionToken(get_session_token) => get_session_token.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<sts::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
