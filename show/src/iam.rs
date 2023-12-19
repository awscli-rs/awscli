use super::*;

impl Show for aws_sdk_iam::types::User {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.user_name()._fmt() } " " { self.user_id()._fmt() }
        ))
    }
}

impl Show for aws_sdk_iam::types::SummaryKeyType {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }
}
