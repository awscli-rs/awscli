use std::collections::HashMap;

use serde_dynamo as dynamo;
use serde_json as json;

use super::*;

#[derive(Debug, Args)]
pub struct GetItem {
    /// The name of the table to describe.
    #[arg(long)]
    table_name: String,
    #[arg(long, value_parser = key_parser)]
    key: HashMap<String, dynamodb::types::AttributeValue>,
}

#[async_trait]
impl Execute for GetItem {
    async fn execute(self: Box<Self>, client: dynamodb::Client) -> DynamoResult {
        let item = client
            .get_item()
            .table_name(self.table_name)
            .set_key(Some(self.key))
            .send()
            .await?
            .item;
        Ok(Box::new(item))
    }
}

fn key_parser(text: &str) -> Result<HashMap<String, dynamodb::types::AttributeValue>, KeyError> {
    let keys = json::from_str::<HashMap<_, dynamo::AttributeValue>>(text)?
        .into_iter()
        .map(to_attribute_value)
        .collect::<Result<_, _>>()?;
    Ok(keys)
}

fn to_attribute_value(
    (key, value): (String, dynamo::AttributeValue),
) -> dynamo::Result<(String, dynamodb::types::AttributeValue)> {
    dynamo::to_attribute_value(value).map(|value| (key, value))
}

#[derive(Debug, thiserror::Error)]
// #[error("{message}")]
enum KeyError {
    #[error(transparent)]
    Json(#[from] json::Error),
    #[error(transparent)]
    Dynamo(#[from] dynamo::Error),
}
