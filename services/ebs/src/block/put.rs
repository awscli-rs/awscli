use std::path::PathBuf;

use super::*;

/// Writes a block of data to a snapshot.
#[derive(Debug, Args)]
pub struct PutSnapshotBlock {
    /// The ID of the snapshot.
    #[arg(long)]
    snapshot_id: String,

    /// The block index of the block in which to write the data.
    #[arg(long)]
    block_index: i32,

    /// The block index of the block in which to read the data.
    #[arg(long)]
    block_data: PathBuf,

    /// The progress of the write process, as a percentage.
    #[arg(long)]
    progress: Option<i32>,
}

impl PutSnapshotBlock {
    pub(crate) async fn execute(self, config: &Config) -> EbsResult {
        let block_data = ebs::primitives::ByteStream::from_path(self.block_data)
            .await
            .map_err(ebs::error::BuildError::other)?;

        let block = config
            .ebs()
            .put_snapshot_block()
            .snapshot_id(self.snapshot_id)
            .block_data(block_data)
            .send()
            .await?;

        Ok(Box::new(block))
    }
}
