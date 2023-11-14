use super::*;

/// Creates a new Amazon EBS snapshot.
#[derive(Debug, Args)]
pub struct StartSnapshot {
    /// The size of the volume, in GiB. The maximum size is 65536 GiB (64 TiB).
    #[arg(long)]
    volume_size: i64,

    /// The ID of the parent snapshot.
    #[arg(long)]
    parent_snapshot_id: Option<String>,

    /// The tags to apply to the snapshot.
    #[arg(long, value_parser = parsers::tag::parse_tags::<ebs::types::Tag>, num_args = 1..)]
    tags: Option<Vec<ebs::types::Tag>>,

    /// A description for the snapshot.
    #[arg(long)]
    description: Option<String>,

    /// A unique, case-sensitive identifier that you provide to ensure
    /// the idempotency of the request.
    #[arg(long)]
    client_token: Option<String>,

    /// The amount of time (in minutes) after which the snapshot is automatically cancelled
    #[arg(long)]
    timeout: Option<i32>,
}

#[async_trait]
impl Execute for StartSnapshot {
    async fn execute(self: Box<Self>, config: &Config) -> EbsResult {
        let output = config
            .client()
            .start_snapshot()
            .volume_size(self.volume_size)
            .set_parent_snapshot_id(self.parent_snapshot_id)
            .set_description(self.description)
            .set_tags(self.tags)
            .set_client_token(self.client_token)
            .set_timeout(self.timeout)
            .send()
            .await?;
        Ok(Box::new(output))
    }
}
