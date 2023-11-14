use super::*;

/// Returns a set of temporary credentials for an Amazon Web Services
/// account or IAM user.
#[derive(Debug, Args)]
pub struct GetSessionToken {
    /// The duration, in seconds, that the credentials should remain valid.
    #[arg(long)]
    duration_seconds: Option<i32>,

    /// The identification number of the MFA device that is associated with
    /// the IAM user who is making the GetSessionToken call.
    #[arg(long)]
    serial_number: Option<String>,

    /// The value provided by the MFA device, if MFA is required.
    #[arg(long)]
    token_code: Option<String>,
}

#[async_trait]
impl Execute for GetSessionToken {
    async fn execute(self: Box<Self>, config: &Config) -> StsResult {
        let credentials = config
            .client()
            .get_session_token()
            .set_duration_seconds(self.duration_seconds)
            .set_serial_number(self.serial_number)
            .set_token_code(self.token_code)
            .send()
            .await?
            .credentials;
        Ok(Box::new(credentials))
    }
}
