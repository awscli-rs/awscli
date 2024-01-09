use aws_config::{BehaviorVersion, ConfigLoader, SdkConfig};
use aws_types::app_name::AppName;
use aws_types::region::Region;

use aws_sdk_account as account;

pub use output::Output;

mod output;

#[derive(Debug)]
pub struct Config {
    debug: bool,
    no_paginate: bool,
    output: Output,
    shared_config: SdkConfig,
}

impl Config {
    pub async fn new(
        debug: bool,
        no_paginate: bool,
        endpoint_url: Option<String>,
        profile: Option<String>,
        region: Option<String>,
        output: Option<Output>,
    ) -> Self {
        let output = output.unwrap_or_default();
        let app_name = AppName::new("raws").expect("valid app name");
        let shared_config = aws_config::defaults(BehaviorVersion::latest())
            .app_name(app_name)
            .optionally_profile(profile)
            .optionally_region(region)
            .optionally_endpoint_url(endpoint_url)
            .load()
            .await;

        Self {
            debug,
            no_paginate,
            output,
            shared_config,
        }
    }

    pub fn config(&self) -> &SdkConfig {
        &self.shared_config
    }

    pub fn no_paginate(&self) -> bool {
        self.no_paginate
    }

    pub fn show(&self, object: Box<dyn show::Show>) {
        let text = if self.debug {
            object.debug()
        } else {
            self.output.output(object)
        };
        fmtools::println!({ text });
    }

    pub fn account(&self) -> account::Client {
        account::Client::new(self.config())
    }
}

trait Optionally {
    fn optionally_profile(self, profile: Option<String>) -> Self;
    fn optionally_region(self, region: Option<String>) -> Self;
    fn optionally_endpoint_url(self, endpoint_url: Option<String>) -> Self;
}

impl Optionally for ConfigLoader {
    fn optionally_profile(self, profile: Option<String>) -> Self {
        if let Some(profile) = profile {
            self.profile_name(profile)
        } else {
            self
        }
    }

    fn optionally_region(self, region: Option<String>) -> Self {
        if let Some(region) = region.map(Region::new) {
            self.region(region)
        } else {
            self
        }
    }

    fn optionally_endpoint_url(self, endpoint_url: Option<String>) -> Self {
        if let Some(endpoint_url) = endpoint_url {
            self.endpoint_url(endpoint_url)
        } else {
            self
        }
    }
}
