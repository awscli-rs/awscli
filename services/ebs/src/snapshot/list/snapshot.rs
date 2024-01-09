use super::*;

/// Returns information about the blocks in an Amazon Elastic Block Store snapshot.
#[derive(Debug, Args)]
pub struct ListSnapshotBlocks {
    /// The ID of the snapshot from which to get block indexes and block tokens.
    #[arg(long)]
    snapshot_id: String,

    /// The token to request the next page of results.
    #[arg(long)]
    next_token: Option<String>,
}

impl ListSnapshotBlocks {
    pub(crate) async fn execute(self, config: &Config) -> EbsResult {
        let blocks = config
            .ebs()
            .list_snapshot_blocks()
            .snapshot_id(self.snapshot_id)
            .set_next_token(self.next_token)
            .into_paginator()
            .send()
            .try_collect()
            .await?
            .into_iter()
            .filter_map(|item| item.blocks)
            .flatten()
            .collect::<Vec<_>>();
        Ok(Box::new(blocks))
    }
}
