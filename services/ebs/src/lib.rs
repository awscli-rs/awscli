use aws_sdk_ebs as ebs;
use clap::{Args, Subcommand};

use config::Config;
use error::RawsError;

mod block;
mod snapshot;

type EbsResult<T = Box<dyn show::Show>> = std::result::Result<T, ebs::Error>;

/// Amazon Elastic Block Store (Amazon EBS) direct API
///
#[derive(Debug, Subcommand)]
pub enum Ebs {
    CompleteSnapshot(snapshot::CompleteSnapshot),
    StartSnapshot(snapshot::StartSnapshot),
    ListSnapshotBlocks(snapshot::ListSnapshotBlocks),
    ListChangedBlocks(snapshot::ListChangedBlocks),
    GetSnapshotBlock(block::GetSnapshotBlock),
    PutSnapshotBlock(block::PutSnapshotBlock),
}

impl Ebs {
    async fn execute(self, config: &Config) -> EbsResult {
        match self {
            Self::CompleteSnapshot(complete_snapshot) => complete_snapshot.execute(config).await,
            Self::StartSnapshot(start_snapshot) => start_snapshot.execute(config).await,
            Self::ListSnapshotBlocks(list_snapshot_blocks) => {
                list_snapshot_blocks.execute(config).await
            }
            Self::ListChangedBlocks(list_changed_blocks) => {
                list_changed_blocks.execute(config).await
            }
            Self::GetSnapshotBlock(get_snapshot_block) => get_snapshot_block.execute(config).await,
            Self::PutSnapshotBlock(put_snapshot_block) => put_snapshot_block.execute(config).await,
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<ebs::Error>> {
        self.execute(&config)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
