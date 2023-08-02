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
}

#[async_trait]
impl Execute for StartSnapshot {
    async fn execute(self: Box<Self>, client: ebs::Client) -> EbsResult {
        let output = client
            .start_snapshot()
            .volume_size(self.volume_size)
            .set_parent_snapshot_id(self.parent_snapshot_id)
            .send()
            .await?;
        Ok(Box::new(output))
    }
}
