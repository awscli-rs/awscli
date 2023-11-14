use super::*;

/// Returns a set of temporary security credentials for users who have been
/// authenticated in a mobile or web application with a web identity provider.
#[derive(Debug, Args)]
pub struct AssumeRoleWithWebIdentity {
    /// The Amazon Resource Name (ARN) of the role that the caller is assuming.
    #[arg(long)]
    role_arn: String,

    /// An identifier for the assumed role session.
    #[arg(long)]
    role_session_name: String,

    /// The OAuth 2.0 access token or OpenID Connect ID token that is provided
    /// by the identity provider.
    #[arg(long)]
    web_identity_token: String,

    /// The fully qualified host component of the domain name of the OAuth
    /// 2.0 identity provider.
    #[arg(long, value_parser = ["www.amazon.com", "graph.facebook.com"])]
    provider_id: Option<String>,

    /// The Amazon Resource Names (ARNs) of the IAM managed policies that
    /// you want to use as managed session policies.
    #[arg(long, value_parser = parsers::sts::parse_arns, num_args = 1..)]
    policy_arns: Option<Vec<sts::types::PolicyDescriptorType>>,

    /// An IAM policy in JSON format that you want to use as an inline session policy.
    #[arg(long)]
    policy: Option<String>,

    /// The duration, in seconds, of the role session.
    #[arg(long)]
    duration_seconds: Option<i32>,
}

#[async_trait]
impl Execute for AssumeRoleWithWebIdentity {
    async fn execute(self: Box<Self>, config: &Config) -> StsResult {
        let credentials = config
            .client()
            .assume_role_with_web_identity()
            .role_arn(self.role_arn)
            .role_session_name(self.role_session_name)
            .web_identity_token(self.web_identity_token)
            .set_provider_id(self.provider_id)
            .set_policy_arns(self.policy_arns)
            .set_policy(self.policy)
            .set_duration_seconds(self.duration_seconds)
            .send()
            .await?
            .credentials;

        Ok(Box::new(credentials))
    }
}
