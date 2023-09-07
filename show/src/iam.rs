use super::*;

impl Show for aws_sdk_iam::types::User {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.user_name()._fmt() } " " { self.user_id()._fmt() }
        ))
    }

    fn text(&self) -> String {
        let username = self.user_name().unwrap_or_default();
        let userid = self.user_id().unwrap_or_default();
        fmtools::format!(
            {username} " " {userid}
        )
    }
}
