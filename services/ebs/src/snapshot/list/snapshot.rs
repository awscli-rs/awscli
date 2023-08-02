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

#[async_trait]
impl Execute for ListSnapshotBlocks {
    async fn execute(self: Box<Self>, client: ebs::Client) -> EbsResult {
        let blocks = client
            .list_snapshot_blocks()
            .snapshot_id(self.snapshot_id)
            .set_next_token(self.next_token)
            .into_paginator()
            .send()
            .collect::<Result<Vec<_>, _>>()
            .await?
            .into_iter()
            .filter_map(|item| item.blocks)
            .flatten()
            .collect::<Vec<_>>();
        Ok(Box::new(blocks))
    }
}