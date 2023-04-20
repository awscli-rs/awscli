use super::*;

impl Show for aws_sdk_dynamodb::types::TableDescription {
    fn show(&self) -> String {
        let tablename = self.table_name().unwrap_or_default();
        let tableid = self.table_id().unwrap_or_default();
        format!("{tablename} {tableid}")
    }

    fn detailed_show(&self) -> String {
        format!("{self:?}")
    }
}
