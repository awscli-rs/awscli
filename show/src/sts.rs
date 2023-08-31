use super::*;

mod accesskey;
mod identity;

impl Show for aws_sdk_sts::types::Credentials {
    fn text(&self) -> String {
        let access_key_id = self.access_key_id().unwrap_or_default();
        let secret_access_key = self.secret_access_key().unwrap_or_default();
        let session_token = self.session_token().unwrap_or_default();
        let expiration = self.expiration().text();
        fmtools::format!(
            "CREDENTIALS\t"
            {access_key_id} "\t"
            {expiration} "\t"
            {secret_access_key} "\t"
            {session_token}
        )
    }
}
