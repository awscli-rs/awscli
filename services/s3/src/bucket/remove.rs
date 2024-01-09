use super::*;

/// Deletes an empty S3 bucket.
#[derive(Debug, Args)]
pub struct Remove {
    /// S3 bucket to delete
    s3_uri: String,

    /// Deletes all objects in the bucket including the
    /// bucket itself. Note that versioned objects will not be deleted in this
    /// process which would cause the bucket deletion to fail because the
    /// bucket would not be empty.
    #[arg(long)]
    force: bool,
}

impl Remove {
    pub(crate) async fn execute(self, config: &Config) -> S3Result {
        let client = S3Client::with_config(config);
        let bucket = bucket_name(&self.s3_uri);

        if self.force {
            client.remove_unversioned_objects(bucket).await?;
        }

        client.delete_bucket(bucket).await?;

        Ok(Box::new(()))
    }
}
