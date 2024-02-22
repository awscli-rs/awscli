
use super::*;

impl Show for aws_smithy_types::DateTime {
    fn _fmt(&self) -> Box<dyn fmt::Display> {
        Box::new(self.to_string())
    }
}
