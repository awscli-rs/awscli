use super::*;

impl Show for aws_sdk_iam::types::User {
    fn show(&self) -> String {
        let username = self.user_name().unwrap_or_default();
        let userid = self.user_id().unwrap_or_default();
        format!("{username} {userid}")
    }

    fn detailed_show(&self) -> String {
        format!("{self:?}")
    }
}
