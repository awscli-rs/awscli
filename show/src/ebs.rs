use super::*;

impl Show for aws_sdk_ebs::types::Status {
    fn text(&self) -> String {
        self.as_str().to_string()
    }
}
