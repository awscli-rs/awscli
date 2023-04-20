use dynamo::types::AttributeDefinition;
use dynamo::types::KeySchemaElement;
use dynamo::types::KeyType;
use dynamo::types::ScalarAttributeType;

use super::*;

#[derive(Debug, Args)]
pub struct CreateTable {
    /// The name of the table to create.
    #[arg(long)]
    table_name: String,
    #[arg(long, required = true, visible_alias = "attrs", value_parser = attribute_definition_parser)]
    attribute_definitions: Vec<AttributeDefinition>,
    #[arg(long, required = true, visible_alias = "keys", value_parser = key_schema_parser)]
    key_schema: Vec<KeySchemaElement>,
}

#[async_trait]
impl Execute for CreateTable {
    async fn execute(self: Box<Self>, client: dynamo::Client) -> DynamoResult {
        let table = client
            .create_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table_description;
        Ok(Box::new(table))
    }
}

fn attribute_definition_parser(
    text: &str,
) -> Result<AttributeDefinition, InvalidAttributeDefinition> {
    let (name, r#type) = split_pair(text, ',')?;
    let attr_name = attr_name(name)?;
    let attr_type = attr_type(r#type)?;
    let attr = AttributeDefinition::builder()
        .attribute_name(attr_name)
        .attribute_type(attr_type)
        .build();
    Ok(attr)
}

fn key_schema_parser(text: &str) -> Result<KeySchemaElement, InvalidAttributeDefinition> {
    let (name, keytype) = split_pair(text, ',')?;
    let attr_name = attr_name(name)?;
    let key_type = key_type(keytype)?;
    let element = KeySchemaElement::builder()
        .attribute_name(attr_name)
        .key_type(key_type)
        .build();
    Ok(element)
}

fn attr_name(text: &str) -> Result<&str, InvalidAttributeDefinition> {
    let (name, value) = split_pair(text, '=')?;
    if name.to_lowercase() != "attributename" {
        Err(InvalidAttributeDefinition::malformed(text))?;
    }

    if value.contains(char::is_whitespace) {
        Err(InvalidAttributeDefinition::attr_name(value))?;
    }

    Ok(value)
}

fn attr_type(text: &str) -> Result<ScalarAttributeType, InvalidAttributeDefinition> {
    let (attr, r#type) = split_pair(text, '=')?;
    if attr.to_lowercase() != "attributetype" {
        Err(InvalidAttributeDefinition::malformed(text))?;
    }

    match r#type.into() {
        ScalarAttributeType::Unknown(_) => Err(InvalidAttributeDefinition::attr_type(r#type)),
        other => Ok(other),
    }
}

fn key_type(text: &str) -> Result<KeyType, InvalidAttributeDefinition> {
    let (key, r#type) = split_pair(text, '=')?;
    if key.to_lowercase() != "keytype" {
        Err(InvalidAttributeDefinition::malformed(text))?;
    }

    match r#type.into() {
        KeyType::Unknown(_) => Err(InvalidAttributeDefinition::attr_type(r#type)),
        other => Ok(other),
    }
}

fn split_pair(text: &str, delimiter: char) -> Result<(&str, &str), InvalidAttributeDefinition> {
    text.split_once(delimiter)
        .ok_or_else(|| InvalidAttributeDefinition::malformed(text))
}

#[derive(Clone, Debug, thiserror::Error)]
pub(crate) enum InvalidAttributeDefinition {
    #[error("Malformed Attribute Definition: '{0}'")]
    Malformed(String),
    #[error("Invalid Attribute Name: '{0}'")]
    Name(String),
    #[error("Invalid Attribute Type: '{0}'")]
    Type(String),
}

impl InvalidAttributeDefinition {
    fn malformed(text: &str) -> Self {
        Self::Malformed(text.to_string())
    }

    fn attr_name(name: &str) -> Self {
        Self::Name(name.to_string())
    }

    fn attr_type(r#type: &str) -> Self {
        Self::Type(r#type.to_string())
    }
}
