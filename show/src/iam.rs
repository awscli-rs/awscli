use super::*;

impl Show for aws_sdk_iam::types::User {
    fn text(&self) -> String {
        let username = self.user_name().unwrap_or_default();
        let userid = self.user_id().unwrap_or_default();
        fmtools::format!(
            {username} " " {userid}
        )
    }

    fn debug(&self) -> String {
        format!("{self:?}")
    }
}
