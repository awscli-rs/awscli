use super::*;

/// Creates an S3 bucket.
#[derive(Debug, Args)]
pub struct Make {
    /// S3 bucket to delete
    s3_uri: String,
}

impl Make {
    pub(crate) async fn execute(self, config: &Config) -> S3Result {
        let client = S3Client::with_config(config);
        let bucket = bucket_name(&self.s3_uri);
        let location = client.create_bucket(bucket).await?;

        Ok(Box::new(location))
    }
}
