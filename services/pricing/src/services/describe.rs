use super::*;

/// Returns the metadata for one service or a list of the metadata for all services.
#[derive(Debug, Args)]
pub struct DescribeServices {
    /// The code for the service whose information you want to retrieve
    #[arg(long)]
    service_code: Option<String>,
    /// The format version that you want the response to be in
    #[arg(long)]
    format_version: Option<String>,
}

#[async_trait]
impl Execute for DescribeServices {
    async fn execute(self: Box<Self>, config: &Config) -> PricingResult {
        let services = config
            .client()
            .describe_services()
            .set_service_code(self.service_code)
            .set_format_version(self.format_version)
            .into_paginator()
            .items()
            .send()
            .try_collect()
            .await?;

        Ok(Box::new(services))
    }
}
