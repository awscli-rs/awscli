use super::*;

/// Returns a list of attribute values.
#[derive(Debug, Args)]
pub struct GetAttributeValues {
    /// The service code for the service whose attributes you want to retrieve.
    #[arg(long)]
    service_code: String,
    /// The name of the attribute that you want to retrieve the values for
    #[arg(long)]
    attribute_name: String,
}

impl GetAttributeValues {
    pub(crate) async fn execute(self, config: &Config) -> PricingResult {
        let values = config
            .pricing()
            .get_attribute_values()
            .service_code(self.service_code)
            .attribute_name(self.attribute_name)
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(values))
    }
}
