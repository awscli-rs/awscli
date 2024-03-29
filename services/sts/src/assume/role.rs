use super::*;

/// Returns a set of temporary security credentials that you can use to
/// access AWS resources
#[derive(Debug, Args)]
pub struct AssumeRole {
    /// The Amazon Resource Name (ARN) of the role to assume.
    #[arg(long)]
    role_arn: String,
    /// An identifier for the assumed role session.
    #[arg(long)]
    role_session_name: String,
}

impl AssumeRole {
    pub(crate) async fn execute(self, config: &Config) -> StsResult {
        let credentials = config
            .sts()
            .assume_role()
            .role_arn(self.role_arn)
            .role_session_name(self.role_session_name)
            .send()
            .await?
            .credentials;

        Ok(Box::new(credentials))
    }
}
