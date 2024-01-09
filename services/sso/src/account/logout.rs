use super::*;

/// Removes all cached AWS SSO access tokens and any cached temporary AWS
/// credentials retrieved with SSO access tokens across all profiles.
#[derive(Debug, Args)]
pub struct Logout;

impl Logout {
    pub(crate) async fn execute(self, config: &Config) -> SsoResult {
        config.sso().logout().send().await?;
        Ok(Box::new(()))
    }
}
