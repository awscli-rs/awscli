use super::*;

/// Returns a set of temporary security credentials for users who have been
/// authenticated via a SAML authentication response.
#[derive(Debug, Args)]
pub struct AssumeRoleWithSaml {
    /// The Amazon Resource Name (ARN) of the role that the caller is assuming.
    #[arg(long)]
    role_arn: String,

    /// The Amazon Resource Name (ARN) of the SAML provider in IAM that describes the IdP.
    #[arg(long)]
    principal_arn: String,

    /// The base64 encoded SAML authentication response provided by the IdP.
    #[arg(long)]
    saml_assertion: String,

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
impl Execute for AssumeRoleWithSaml {
    async fn execute(self: Box<Self>, client: sts::Client) -> StsResult {
        let credentials = client
            .assume_role_with_saml()
            .role_arn(self.role_arn)
            .principal_arn(self.principal_arn)
            .saml_assertion(self.saml_assertion)
            .set_policy_arns(self.policy_arns)
            .set_policy(self.policy)
            .set_duration_seconds(self.duration_seconds)
            .send()
            .await?
            .credentials;

        Ok(Box::new(credentials))
    }
}
