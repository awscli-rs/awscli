use async_trait::async_trait;
use aws_sdk_pricing as pricing;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod pricelist;
mod services;

type PricingResult<T = Box<dyn show::Show>> = std::result::Result<T, pricing::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, config: &Config) -> PricingResult;
}

trait ClientExt {
    fn client(&self) -> pricing::Client;
}

impl ClientExt for Config {
    fn client(&self) -> pricing::Client {
        pricing::Client::new(self.config())
    }
}

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
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::DescribeServices(describe_services) => Box::new(describe_services),
            Self::GetAttributeValues(get_attribute_values) => Box::new(get_attribute_values),
            Self::GetProducts(get_products) => Box::new(get_products),
            Self::GetPriceListFileUrl(get_price_list_file_url) => Box::new(get_price_list_file_url),
            Self::ListPriceLists(list_price_lists) => Box::new(list_price_lists),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<pricing::Error>> {
        self.boxed()
            .execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
