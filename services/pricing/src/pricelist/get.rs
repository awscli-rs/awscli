use super::*;

/// This returns the URL that you can retrieve your Price List file from.
#[derive(Debug, Args)]
pub struct GetPriceListFileUrl {
    /// The unique identifier that maps to where your Price List files are located.
    #[arg(long)]
    price_list_arn: String,
    /// The format that you want to retrieve your Price List files in.
    #[arg(long)]
    file_format: String,
}

#[async_trait]
impl Execute for GetPriceListFileUrl {
    async fn execute(self: Box<Self>, config: &Config) -> PricingResult {
        let url = config
            .client()
            .get_price_list_file_url()
            .price_list_arn(self.price_list_arn)
            .file_format(self.file_format)
            .send()
            .await?
            .url;

        Ok(Box::new(url))
    }
}
