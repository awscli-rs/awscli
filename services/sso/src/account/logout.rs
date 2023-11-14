use super::*;

/// Removes all cached AWS SSO access tokens and any cached temporary AWS
/// credentials retrieved with SSO access tokens across all profiles.
#[derive(Debug, Args)]
pub struct Logout;

#[async_trait]
impl Execute for Logout {
    async fn execute(self: Box<Self>, config: &Config) -> SsoResult {
        config.client().logout().send().await?;
        Ok(Box::new(()))
    }
}
