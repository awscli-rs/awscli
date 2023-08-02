use std::error::Error as StdError;
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

#[async_trait]
impl Execute for GetSnapshotBlock {
    async fn execute(self: Box<Self>, client: ebs::Client) -> EbsResult {
        let block = client
            .get_snapshot_block()
            .snapshot_id(self.snapshot_id)
            .block_index(self.block_index)
            .block_token(self.block_token)
            .send()
            .await?;

        let contents = block
            .block_data
            .collect()
            .await
            .map(|bytes| bytes.into_bytes())
            .map_err(unhandled)?;

        tokio::fs::write(self.outfile, contents)
            .await
            .map_err(unhandled)?;
        let checksum = block.checksum;
        let checksum_algo = block.checksum_algorithm;
        let data_length = block.data_length;

        Ok(Box::new((checksum, checksum_algo, data_length)))
    }
}

fn unhandled(source: impl Into<Box<dyn StdError + Send + Sync + 'static>>) -> ebs::Error {
    let unhandled = aws_smithy_types::error::Unhandled::builder()
        .source(source)
        .build();
    ebs::Error::Unhandled(unhandled)
}
