use super::*;

impl Show for aws_sdk_sso::types::AccountInfo {
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
