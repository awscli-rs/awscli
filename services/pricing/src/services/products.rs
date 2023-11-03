use super::*;

/// Returns a list of all products that match the filter criteria.
#[derive(Debug, Args)]
pub struct GetProducts {
    /// The service code for the service whose attributes you want to retrieve.
    #[arg(long)]
    service_code: String,
    /// The list of filters that limit the returned products.
    #[arg(long)]
    filter: Option<String>,
    /// The format version that you want the response to be in
    #[arg(long)]
    format_version: Option<String>,
}

#[async_trait]
impl Execute for GetProducts {
    async fn execute(self: Box<Self>, client: pricing::Client) -> PricingResult {
        let values = client
            .get_products()
            .service_code(self.service_code)
            .set_filters(None)
            .set_format_version(self.format_version)
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(values))
    }
}
