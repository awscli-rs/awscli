use super::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct CallerIdentity {
    user_id: String,
    account: String,
    arn: String,
}

impl Show for aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityOutput {
    fn text(&self) -> String {
        fmtools::format!({ CallerIdentity::from(self) })
    }

    fn json(&self) -> String {
        let identity = CallerIdentity::from(self);
        json::to_string_pretty(&identity).unwrap_or_default()
    }
}

impl fmt::Display for CallerIdentity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_args!("{} {} {}", self.account, self.user_id, self.arn).fmt(f)
    }
}

impl From<&aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityOutput>
    for CallerIdentity
{
    fn from(output: &aws_sdk_sts::operation::get_caller_identity::GetCallerIdentityOutput) -> Self {
        let user_id = output.user_id().unwrap_or_default().to_string();
        let account = output.account().unwrap_or_default().to_string();
        let arn = output.arn().unwrap_or_default().to_string();
        Self {
            user_id,
            account,
            arn,
        }
    }
}
