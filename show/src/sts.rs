use super::*;

mod accesskey;
mod identity;

impl Show for aws_sdk_sts::types::Credentials {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "CREDENTIALS\t"
                {self.access_key_id()._fmt()} "\t"
                {self.expiration()._fmt()} "\t"
                {self.secret_access_key()._fmt()} "\t"
                {self.session_token()._fmt()}
        ))
    }
}
