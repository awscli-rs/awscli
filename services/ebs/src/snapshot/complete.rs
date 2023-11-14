use super::*;

/// Seals and completes the snapshot after all of the required blocks of data have been written to it.
#[derive(Debug, Args)]
pub struct CompleteSnapshot {
    /// The ID of the snapshot.
    #[arg(long)]
    snapshot_id: String,

    /// The number of blocks that were written to the snapshot.
    #[arg(long)]
    changed_block_count: i32,
}

#[async_trait]
impl Execute for CompleteSnapshot {
    async fn execute(self: Box<Self>, config: &Config) -> EbsResult {
        let status = config
            .client()
            .complete_snapshot()
            .snapshot_id(self.snapshot_id)
            .changed_blocks_count(self.changed_block_count)
            .send()
            .await?
            .status;
        Ok(Box::new(status))
    }
}
