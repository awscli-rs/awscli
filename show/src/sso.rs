use super::*;

impl Show for aws_sdk_sso::types::AccountInfo {
    fn text(&self) -> String {
        let account_id = self.account_id().unwrap_or_default();
        let account_name = self.account_name().unwrap_or_default();
        let email_address = self.email_address().unwrap_or_default();

        fmtools::format!({account_id} " " {account_name} " " {email_address})
    }

    fn detailed_show(&self) -> String {
        format!("{self:?}")
    }
}
