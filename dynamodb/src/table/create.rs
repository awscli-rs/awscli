use dynamodb::types::AttributeDefinition;
use dynamodb::types::ScalarAttributeType;

use super::*;

#[derive(Debug, Args)]
pub struct CreateTable {
    /// The name of the table to create.
    #[arg(long)]
    table_name: String,
    #[arg(long, required = true, visible_alias = "attrs", value_parser = attribute_definition_parser)]
    attribute_definitions: Vec<AttributeDefinition>,
}

impl CreateTable {
    pub async fn execute(self, client: dynamodb::Client) -> DynamoResult<Option<TableDescription>> {
        let table = client
            .create_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table_description;
        Ok(table)
    }
}

fn attribute_definition_parser(
    text: &str,
) -> Result<AttributeDefinition, InvalidAttributeDefinition> {
    let (name, r#type) = attribute_definition(text)?;
    let attr_name = attr_name(name)?;
    let attr_type = attr_type(r#type)?;
    let attr = AttributeDefinition::builder()
        .attribute_name(attr_name)
        .attribute_type(attr_type)
        .build();
    Ok(attr)
}

fn attr_name(text: &str) -> Result<&str, InvalidAttributeDefinition> {
    let (name, value) = key_value(text)?;
    if name.to_lowercase() != "attributename" {
        Err(InvalidAttributeDefinition::malformed(text))?;
    }

    if value.contains(char::is_whitespace) {
        Err(InvalidAttributeDefinition::attr_name(value))?;
    }

    Ok(value)
}

fn attr_type(text: &str) -> Result<ScalarAttributeType, InvalidAttributeDefinition> {
    let (attr, r#type) = key_value(text)?;
    if attr.to_lowercase() != "attributetype" {
        Err(InvalidAttributeDefinition::malformed(text))?;
    }

    match r#type.into() {
        ScalarAttributeType::Unknown(_) => Err(InvalidAttributeDefinition::attr_type(r#type)),
        other => Ok(other),
    }
}

fn key_value(text: &str) -> Result<(&str, &str), InvalidAttributeDefinition> {
    text.split_once('=')
        .ok_or_else(|| InvalidAttributeDefinition::malformed(text))
}

fn attribute_definition(text: &str) -> Result<(&str, &str), InvalidAttributeDefinition> {
    text.split_once(',')
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
