use aws_sdk_dynamodb::types::AttributeValue;
// use serde_dynamo as dynamo;

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

impl Show for HashMap<String, AttributeValue> {
    fn show(&self) -> String {
        fmtools::format!(
            for item in self {
                {item.0} ": " {item.1:?}
            }
        )
    }

    fn detailed_show(&self) -> String {
        fmtools::format!(
            for item in self {
                {item.0} ": " {item.1:?}
            }
        )
    }

    // fn json(&self) -> String {
    //     let value: json::Value = dynamo::from_item(self).unwrap_or_default();
    //     json::to_string(&value).unwrap_or_default()
    // }

    fn table(&self) -> String {
        "Table view is not implemented yet".to_string()
    }

    fn yaml(&self) -> String {
        "YAML view is not implemented yet".to_string()
    }

    fn yaml_stream(&self) -> String {
        "YAML stream view is not implemented yet".to_string()
    }
}
