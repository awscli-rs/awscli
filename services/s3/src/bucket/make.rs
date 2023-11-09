use super::*;

/// Creates an S3 bucket.
#[derive(Debug, Args)]
pub struct Make {
    /// S3 bucket to delete
    s3_uri: String,
}

#[async_trait]
impl Execute for Make {
    async fn execute(self: Box<Self>, client: s3::Client) -> S3Result {
        let client = S3Client::from(client);
        let bucket = bucket_name(&self.s3_uri);
        let location = client.create_bucket(bucket).await?;

        Ok(Box::new(location))
    }
}
