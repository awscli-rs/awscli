use super::*;

impl Show for aws_sdk_s3::types::Bucket {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.creation_date()._fmt() } "\t" { self.name()._fmt() }
        ))
    }
}

impl Show for aws_sdk_s3::types::Object {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
             { self.last_modified()._fmt() } " "
             { self.size()._fmt() } " "
             { self.key()._fmt() }
        ))
    }
}
