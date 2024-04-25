use aws_sdk_pricing as pricing;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod pricelist;
mod services;

type PricingResult<T = Box<dyn show::Show>> = Result<T, pricing::Error>;

/// AWS Pricing Information
#[derive(Debug, Subcommand)]
pub enum Pricing {
    DescribeServices(services::DescribeServices),
    GetAttributeValues(services::GetAttributeValues),
    GetProducts(services::GetProducts),
    GetPriceListFileUrl(pricelist::GetPriceListFileUrl),
    ListPriceLists(pricelist::ListPriceLists),
}

impl Pricing {
    async fn execute(self, config: &Config) -> PricingResult {
        match self {
            Self::DescribeServices(describe_services) => describe_services.execute(config).await,
            Self::GetAttributeValues(get_attribute_values) => {
                get_attribute_values.execute(config).await
            }
            Self::GetProducts(get_products) => get_products.execute(config).await,
            Self::GetPriceListFileUrl(get_price_list_file_url) => {
                get_price_list_file_url.execute(config).await
            }
            Self::ListPriceLists(list_price_lists) => list_price_lists.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<pricing::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
