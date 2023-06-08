use aws_config::{ConfigLoader, SdkConfig};
use aws_types::region::Region;

pub use output::Output;

mod output;

#[derive(Debug)]
pub struct Config {
    debug: bool,
    output: Output,
    shared_config: SdkConfig,
}

impl Config {
    pub async fn new(
        debug: bool,
        profile: Option<String>,
        region: Option<String>,
        output: Option<Output>,
    ) -> Self {
        let output = output.unwrap_or_default();
        let shared_config = aws_config::from_env()
            .optionally_profile(profile)
            .optionally_region(region)
            .load()
            .await;

        Self {
            debug,
            output,
            shared_config,
        }
    }

    pub fn config(&self) -> &SdkConfig {
        &self.shared_config
    }

    pub fn show(&self, object: Box<dyn show::Show>) {
        let text = if self.debug {
            object.debug()
        } else {
            self.output.output(object)
        };
        fmtools::println!({ text });
    }
}

trait Optionally {
    fn optionally_profile(self, profile: Option<String>) -> Self;
    fn optionally_region(self, region: Option<String>) -> Self;
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
}
