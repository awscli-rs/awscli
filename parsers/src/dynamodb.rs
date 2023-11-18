use aws_sdk_dynamodb::types::AttributeDefinition;
use aws_sdk_dynamodb::types::KeySchemaElement;
use aws_sdk_dynamodb::types::KeyType;
use aws_sdk_dynamodb::types::ScalarAttributeType;

use super::*;

pub fn attribute_definition_parser(text: &str) -> Result<AttributeDefinition, BuildError> {
    let (name, r#type) = split_pair(text, ',')?;
    let attr_name = attr_name(name)?;
    let attr_type = attr_type(r#type)?;
    AttributeDefinition::builder()
        .attribute_name(attr_name)
        .attribute_type(attr_type)
        .build()
}

pub fn key_schema_parser(text: &str) -> Result<KeySchemaElement, BuildError> {
    let (name, keytype) = split_pair(text, ',')?;
    let attr_name = attr_name(name)?;
    let key_type = key_type(keytype)?;
    KeySchemaElement::builder()
        .attribute_name(attr_name)
        .key_type(key_type)
        .build()
}

fn attr_name(text: &str) -> Result<&str, BuildError> {
    let (name, value) = split_pair(text, '=')?;
    if name.to_lowercase() != "attributename" {
        Err(malformed(text))?;
    }

    if value.contains(char::is_whitespace) {
        Err(invalid_attribute_name(value))?;
    }

    Ok(value)
}

fn attr_type(text: &str) -> Result<ScalarAttributeType, BuildError> {
    let (attr, r#type) = split_pair(text, '=')?;
    if attr.to_lowercase() != "attributetype" {
        Err(invalid_attribute_name("AttributeType"))?;
    }

    ScalarAttributeType::try_parse(r#type).map_err(BuildError::other)
}

fn key_type(text: &str) -> Result<KeyType, BuildError> {
    let (key, r#type) = split_pair(text, '=')?;
    if key.to_lowercase() != "keytype" {
        Err(malformed(text))?;
    }

    KeyType::try_parse(r#type).map_err(BuildError::other)
}

fn split_pair(text: &str, delimiter: char) -> Result<(&str, &str), BuildError> {
    text.split_once(delimiter).ok_or_else(|| malformed(text))
}

fn malformed(text: &str) -> BuildError {
    BuildError::other(format!("Malformed Attribute Definition: '{text}'"))
}

fn invalid_attribute_name(name: &str) -> BuildError {
    BuildError::other(format!("Invalid Attribute Name: {name})"))
}
