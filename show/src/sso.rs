use super::*;

impl Show for aws_sdk_sso::types::AccountInfo {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("ACCOUNT_ID", self.account_id()) } "\n"
            { prefixed_item("ACCOUNT_NAME", self.account_name()) } "\n"
            { prefixed_item("EMAIL", self.email_address()) }
        ))
    }

    fn text(&self) -> String {
        let account_id = self.account_id().unwrap_or_default();
        let account_name = self.account_name().unwrap_or_default();
        let email_address = self.email_address().unwrap_or_default();

        fmtools::format!(
            "ACCOUNT_ID " {account_id} "\n"
            "ACCOUNT_NAME " {account_name} "\n"
            "EMAIL " {email_address}
        )
    }
}

impl Show for aws_sdk_sso::types::RoleInfo {
    fn _fmt(&self) -> Box<dyn fmt::Display> {
        Box::new(fmtools::format!(
            { prefixed_item("ACCOUNT_ID", self.account_id()) } "\n"
            { prefixed_item("ROLE_NAME", self.role_name()) }
        ))
    }

    fn text(&self) -> String {
        let account_id = self.account_id().unwrap_or_default();
        let role_name = self.role_name().unwrap_or_default();

        fmtools::format!(
            "ACCOUNT_ID " {account_id} "\n"
            "ROLE_NAME " {role_name}
        )
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

    fn text(&self) -> String {
        let access_key_id = self.access_key_id().unwrap_or_default();
        let secret_access_key = self.secret_access_key().unwrap_or_default();
        let session_token = self.session_token().unwrap_or_default();
        let expiration = self.expiration();

        fmtools::format!(
            "ACCESS_KEY_ID " {access_key_id} "\n"
            "SECRET_ACCESS_KEY " {secret_access_key} "\n"
            "SESSION_TOKEN "{session_token} "\n"
            "EXPIRATION " {expiration}
        )
    }
}
