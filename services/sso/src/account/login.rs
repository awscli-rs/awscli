use super::*;

/// Retrieves and caches an AWS SSO access token to exchange for AWS credentials.
#[derive(Debug, Args)]
pub struct Login {
    /// Disables automatically opening the verfication URL in the default browser.
    #[arg(long)]
    no_browse: bool,
    /// An explicit SSO session to use to login.
    #[arg(long)]
    sso_session: Option<String>,
}

#[async_trait]
impl Execute for Login {
    async fn execute(self: Box<Self>, client: sso::Client) -> SsoResult {
        let _credentials = client.config().identity_cache().unwrap();

        todo!(r#""sso login" functionality is not implemented yet"#)
    }
}
