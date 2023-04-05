pub async fn load() -> aws_config::SdkConfig {
    aws_config::load_from_env().await
}
