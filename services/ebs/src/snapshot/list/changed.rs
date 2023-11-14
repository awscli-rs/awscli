use super::*;

/// Returns information about the blocks that are different between two Amazon
/// EBS snapshots of the same volume/snapshot lineage.
#[derive(Debug, Args)]
pub struct ListChangedBlocks {
    /// The ID of the first snapshot to use for the comparison.
    #[arg(long)]
    first_snapshot_id: String,

    /// The ID of the second snapshot to use for the comparison.
    #[arg(long)]
    second_snapshot_id: String,

    /// The token to request the next page of results.
    #[arg(long)]
    next_token: Option<String>,
}

#[async_trait]
impl Execute for ListChangedBlocks {
    async fn execute(self: Box<Self>, config: &Config) -> EbsResult {
        let changed_blocks = config
            .client()
            .list_changed_blocks()
            .first_snapshot_id(self.first_snapshot_id)
            .second_snapshot_id(self.second_snapshot_id)
            .set_next_token(self.next_token)
            .into_paginator()
            .send()
            .try_collect()
            .await?
            .into_iter()
            .filter_map(|item| item.changed_blocks)
            .flatten()
            .collect::<Vec<_>>();
        Ok(Box::new(changed_blocks))
    }
}
