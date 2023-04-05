use aws_config::SdkConfig;

pub struct Config {
    shared_config: SdkConfig,
}

impl Config {
    pub async fn load() -> Self {
        let shared_config = aws_config::load_from_env().await;
        Self { shared_config }
    }

    pub fn config(&self) -> &SdkConfig {
        &self.shared_config
    }
}
