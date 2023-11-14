use super::*;

/// Lists all roles that are assigned to the user for a given AWS account.
#[derive(Debug, Args)]
pub struct GetRoleCredentials {
    /// The friendly name of the role that is assigned to the user.
    #[arg(long)]
    role_name: String,
    /// The token issued by the CreateToken API call.
    #[arg(long)]
    access_token: String,
    /// The identifier for the AWS account that is assigned to the user.
    #[arg(long)]
    account_id: String,
}

#[async_trait]
impl Execute for GetRoleCredentials {
    async fn execute(self: Box<Self>, config: &Config) -> SsoResult {
        let role_credentials = config
            .client()
            .get_role_credentials()
            .role_name(self.role_name)
            .access_token(self.access_token)
            .account_id(self.account_id)
            .send()
            .await?
            .role_credentials;

        Ok(Box::new(role_credentials))
    }
}
