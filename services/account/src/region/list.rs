use super::*;

/// Lists all the Regions for a given account and their respective opt-in statuses.
#[derive(Debug, Args)]
pub struct ListRegions {
    /// Specifies the 12-digit account ID number of the Amazon Web Services
    /// account that you want to access or modify with this operation.
    #[arg(long)]
    account_id: Option<String>,

    #[arg(long, value_parser = clap::value_parser!(account::types::RegionOptStatus))]
    region_opt_status_contains: Option<Vec<account::types::RegionOptStatus>>,
}

#[async_trait]
impl Execute for ListRegions {
    async fn execute(self: Box<Self>, config: &Config) -> AccountResult {
        let regions = config
            .client()
            .list_regions()
            .set_region_opt_status_contains(self.region_opt_status_contains)
            .into_paginator()
            // This paginator doesn't have `.items()` at least in 0.28
            //.items()
            .send()
            .try_collect()
            .await?
            .into_iter()
            .filter_map(|output| output.regions)
            .flatten()
            .collect::<Vec<_>>();

        Ok(Box::new(regions))
    }
}
