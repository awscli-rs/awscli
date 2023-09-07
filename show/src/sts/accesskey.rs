use super::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Account {
    account: String,
}

impl Show for Account {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!({
            prefixed_item("ACCOUNT", Some(&self.account))
        }))
    }

    fn json(&self) -> String {
        json::to_string_pretty(self).unwrap_or_default()
    }
}

impl Show for aws_sdk_sts::operation::get_access_key_info::GetAccessKeyInfoOutput {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        prefixed_item("ACCOUNT", self.account.as_deref())
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
