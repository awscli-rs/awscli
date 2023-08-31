use super::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Account {
    account: String,
}

impl Show for Account {
    fn text(&self) -> String {
        fmtools::format!("ACCOUNT\t" {self.account})
    }

    fn json(&self) -> String {
        json::to_string_pretty(self).unwrap_or_default()
    }
}

impl Show for aws_sdk_sts::operation::get_access_key_info::GetAccessKeyInfoOutput {
    fn text(&self) -> String {
        Account::from(self).text()
    }

    fn json(&self) -> String {
        Account::from(self).json()
    }
}

impl From<&aws_sdk_sts::operation::get_access_key_info::GetAccessKeyInfoOutput> for Account {
    fn from(info: &aws_sdk_sts::operation::get_access_key_info::GetAccessKeyInfoOutput) -> Self {
        let account = info.account().unwrap_or_default().to_string();
        Self { account }
    }
}
