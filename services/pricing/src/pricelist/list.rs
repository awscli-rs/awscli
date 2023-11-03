use aws_smithy_types::DateTime;

use super::*;

/// This returns a list of Price List references that the requester if authorized to view
#[derive(Debug, Args)]
pub struct ListPriceLists {
    /// The service code or the Savings Plan service code for the attributes that you want to retrieve
    #[arg(long)]
    service_code: String,
    /// The date that the Price List file prices are effective from.
    #[arg(long, value_parser = parsers::datetime::datetime_parser)]
    effective_date: DateTime,
    /// This is used to filter the Price List by Amazon Web Services Region.
    #[arg(long)]
    region_code: Option<String>,
    /// The three alphabetical character ISO-4217 currency code that the Price List files are denominated in.
    #[arg(long)]
    currency_code: String,
}

#[async_trait]
impl Execute for ListPriceLists {
    async fn execute(self: Box<Self>, client: pricing::Client) -> PricingResult {
        let price_lists = client
            .list_price_lists()
            .service_code(self.service_code)
            .effective_date(self.effective_date)
            .set_region_code(self.region_code)
            .currency_code(self.currency_code)
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(price_lists))
    }
}
