use super::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Account {
    account: String,
}

impl Show for aws_sdk_sts::operation::get_access_key_info::GetAccessKeyInfoOutput {
    fn text(&self) -> String {
        let account = Account::from(self);
        fmtools::format!({ account.account })
    }

    fn detailed_show(&self) -> String {
        todo!()
    }

    fn json(&self) -> String {
        let account = Account::from(self);
        json::to_string_pretty(&account).unwrap_or_default()
    }
}

impl From<&aws_sdk_sts::operation::get_access_key_info::GetAccessKeyInfoOutput> for Account {
    fn from(info: &aws_sdk_sts::operation::get_access_key_info::GetAccessKeyInfoOutput) -> Self {
        let account = info.account().unwrap_or_default().to_string();
        Self { account }
    }
}
