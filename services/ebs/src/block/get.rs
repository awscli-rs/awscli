use std::mem;
use std::path::PathBuf;

use super::*;

/// Returns the data in a block in an Amazon Elastic Block Store snapshot.
#[derive(Debug, Args)]
pub struct GetSnapshotBlock {
    /// The ID of the snapshot containing the block from which to get data.
    #[arg(long)]
    snapshot_id: String,

    /// The block index of the block in which to read the data.
    #[arg(long)]
    block_index: i32,

    /// The block index of the block in which to read the data.
    #[arg(long)]
    block_token: String,

    outfile: PathBuf,
}

impl GetSnapshotBlock {
    pub(crate) async fn execute(self, config: &Config) -> EbsResult {
        let mut block = config
            .ebs()
            .get_snapshot_block()
            .snapshot_id(self.snapshot_id)
            .block_index(self.block_index)
            .block_token(self.block_token)
            .send()
            .await?;

        let contents = mem::take(&mut block.block_data)
            .collect()
            .await
            .map(|bytes| bytes.into_bytes())
            .map_err(ebs::error::BuildError::other)?;

        tokio::fs::write(self.outfile, contents)
            .await
            .map_err(ebs::error::BuildError::other)?;

        Ok(Box::new(block))
    }
}
