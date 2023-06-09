use super::*;

mod accesskey;
mod identity;

impl Show for aws_sdk_sts::operation::assume_role::AssumeRoleOutput {
    fn text(&self) -> String {
        self.credentials().text()
    }
}

impl Show for aws_sdk_sts::types::Credentials {
    fn text(&self) -> String {
        let access_key_id = self.access_key_id().unwrap_or_default();
        let secret_access_key = self.secret_access_key().unwrap_or_default();
        let session_token = self.session_token().unwrap_or_default();
        let expiration = self.expiration();
        fmtools::format!(
            {access_key_id} " " {secret_access_key} " " {session_token} " " {expiration:?}
        )
    }
}

impl Show for &aws_sdk_sts::types::Credentials {
    fn text(&self) -> String {
        let access_key_id = self.access_key_id().unwrap_or_default();
        let secret_access_key = self.secret_access_key().unwrap_or_default();
        let session_token = self.session_token().unwrap_or_default();
        let expiration = self.expiration();
        fmtools::format!(
            {access_key_id} " " {secret_access_key} " " {session_token} " " {expiration:?}
        )
    }
}
