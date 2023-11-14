use super::*;

/// Returns a set of temporary security credentials (consisting of an
/// access key ID, a secret access key, and a security token) for a user.
#[derive(Debug, Args)]
pub struct GetFederationToken {
    /// The name of the federated user.
    #[arg(long)]
    name: String,

    /// An IAM policy in JSON format that you want to use as an inline session policy.
    #[arg(long)]
    policy: Option<String>,

    /// The Amazon Resource Names (ARNs) of the IAM managed policies that
    /// you want to use as managed session policies.
    #[arg(long, value_parser = parsers::sts::parse_arns, num_args = 1..)]
    policy_arns: Option<Vec<sts::types::PolicyDescriptorType>>,

    /// The duration, in seconds, that the credentials should remain valid.
    #[arg(long)]
    duration_seconds: Option<i32>,

    /// A list of session tags.
    #[arg(long, value_parser = parsers::tag::parse_tags::<sts::types::Tag>, num_args = 1..)]
    tags: Option<Vec<sts::types::Tag>>,
}

#[async_trait]
impl Execute for GetFederationToken {
    async fn execute(self: Box<Self>, config: &Config) -> StsResult {
        let credentials = config
            .client()
            .get_federation_token()
            .name(self.name)
            .set_policy(self.policy)
            .set_policy_arns(self.policy_arns)
            .set_duration_seconds(self.duration_seconds)
            .set_tags(self.tags)
            .send()
            .await?
            .credentials;
        Ok(Box::new(credentials))
    }
}
