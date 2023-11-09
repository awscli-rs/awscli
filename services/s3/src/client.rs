use super::*;

type S3Result<T> = Result<T, s3::Error>;

#[derive(Debug)]
pub(crate) struct S3Client {
    client: s3::Client,
}

impl S3Client {
    pub(crate) async fn list_buckets(&self) -> S3Result<Vec<s3::types::Bucket>> {
        let buckets = self
            .client
            .list_buckets()
            .send()
            .await?
            .buckets
            .unwrap_or_default();

        Ok(buckets)
    }

    pub(crate) async fn list_all_objects(&self, bucket: &str) -> S3Result<Vec<s3::types::Object>> {
        self.list_objects_v2(bucket, Some(String::from('/')), None)
            .await
    }

    pub(crate) async fn list_objects_v2(
        &self,
        bucket: &str,
        delimiter: Option<String>,
        prefix: Option<String>,
    ) -> S3Result<Vec<s3::types::Object>> {
        let objects = self
            .client
            .list_objects_v2()
            .bucket(bucket)
            .set_delimiter(delimiter)
            .set_prefix(prefix)
            .into_paginator()
            .send()
            .try_collect()
            .await?
            .into_iter()
            .flat_map(|output| output.contents.unwrap_or_default())
            .collect::<Vec<_>>();

        Ok(objects)
    }

    pub(crate) async fn create_bucket(&self, bucket: &str) -> S3Result<Option<String>> {
        let location = self
            .client
            .create_bucket()
            .bucket(bucket)
            .send()
            .await?
            .location;

        Ok(location)
    }

    pub(crate) async fn delete_bucket(&self, bucket: &str) -> S3Result<()> {
        self.client.delete_bucket().bucket(bucket).send().await?;

        Ok(())
    }

    pub(crate) async fn remove_unversioned_objects(
        &self,
        bucket: &str,
    ) -> S3Result<s3::operation::delete_objects::DeleteObjectsOutput> {
        let output = self.client.delete_objects().bucket(bucket).send().await?;

        Ok(output)
    }
}

impl From<s3::Client> for S3Client {
    fn from(client: s3::Client) -> Self {
        Self { client }
    }
}
