use super::*;

/// List S3 objects and common prefixes under a prefix or all S3 buckets.
#[derive(Debug, Args)]
pub struct List {
    s3_uri: Option<String>,
}

impl List {
    pub(crate) async fn execute(self, config: &Config) -> S3Result {
        let client = S3Client::with_config(config);
        if let Some(bucket) = maybe_bucket_name(self.s3_uri.as_deref()) {
            let objects = client.list_all_objects(bucket).await?;
            Ok(Box::new(objects))
        } else {
            let buckets = client.list_buckets().await?;
            Ok(Box::new(buckets))
        }
    }
}
