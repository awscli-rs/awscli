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

impl Login {
    pub(crate) async fn execute(self, config: &Config) -> SsoResult {
        let _credentials = config.sso().config().identity_cache().unwrap();

        todo!(r#""sso login" functionality is not implemented yet"#)
    }
}
