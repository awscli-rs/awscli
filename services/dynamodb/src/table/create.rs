use types::AttributeDefinition;
use types::KeySchemaElement;

use super::*;

#[derive(Debug, Args)]
pub struct CreateTable {
    /// The name of the table to create.
    #[arg(long)]
    table_name: String,
    #[arg(long, required = true, visible_alias = "attrs", value_parser = parsers::dynamodb::attribute_definition_parser)]
    attribute_definitions: Vec<AttributeDefinition>,
    #[arg(long, required = true, visible_alias = "keys", value_parser = parsers::dynamodb::key_schema_parser)]
    key_schema: Vec<KeySchemaElement>,
}

impl CreateTable {
    pub(crate) async fn execute(self, config: &Config) -> DynamoResult {
        let table = config
            .dynamodb()
            .create_table()
            .table_name(self.table_name)
            .send()
            .await?
            .table_description;
        Ok(Box::new(table))
    }
}
