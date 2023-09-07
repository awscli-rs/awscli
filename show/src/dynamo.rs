use super::*;

impl Show for aws_sdk_dynamodb::types::TableDescription {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.table_name()._fmt() } " "
            { self.table_id()._fmt() }
        ))
    }

    fn text(&self) -> String {
        let tablename = self.table_name().unwrap_or_default();
        let tableid = self.table_id().unwrap_or_default();
        fmtools::format!({tablename} " " {tableid})
    }
}
