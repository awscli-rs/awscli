use super::*;

impl Show for aws_sdk_sso::types::AccountInfo {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("ACCOUNT_ID", self.account_id()) } "\n"
            { prefixed_item("ACCOUNT_NAME", self.account_name()) } "\n"
            { prefixed_item("EMAIL", self.email_address()) }
        ))
    }
}

impl Show for aws_sdk_sso::types::RoleInfo {
    fn _fmt(&self) -> Box<dyn fmt::Display> {
        Box::new(fmtools::format!(
            { prefixed_item("ACCOUNT_ID", self.account_id()) } "\n"
            { prefixed_item("ROLE_NAME", self.role_name()) }
        ))
    }
}

impl Show for aws_sdk_sso::types::RoleCredentials {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("ACCESS_KEY_ID", self.access_key_id()) } "\n"
            { prefixed_item("SECRET_ACCESS_KEY", self.secret_access_key()) } "\n"
            { prefixed_item("SESSION_TOKEN", self.session_token()) } "\n"
            { prefixed_item0("EXPIRATION", self.expiration()) }
        ))
    }
}
