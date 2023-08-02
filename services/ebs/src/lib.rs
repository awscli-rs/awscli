use async_trait::async_trait;
use aws_sdk_ebs as ebs;
use clap::{Args, Subcommand};
use tokio_stream::StreamExt;

use config::Config;
use error::RawsError;

mod block;
mod snapshot;

type EbsResult<T = Box<dyn show::Show>> = std::result::Result<T, ebs::Error>;

#[async_trait]
pub trait Execute {
    async fn execute(self: Box<Self>, client: ebs::Client) -> EbsResult;
}

/// Amazon Elastic Block Store (Amazon EBS) direct API
///
#[derive(Debug, Subcommand)]
pub enum Ebs {
    CompleteSnapshot(snapshot::CompleteSnapshot),
    StartSnapshot(snapshot::StartSnapshot),
    ListSnapshotBlocks(snapshot::ListSnapshotBlocks),
    ListChangedBlocks(snapshot::ListChangedBlocks),
    GetSnapshotBlock(block::GetSnapshotBlock),
    // DescribeUpdate(update::DescribeUpdate),
    // ListAddons(addon::ListAddons),
    // DescribeAddon(addon::DescribeAddon),
}

impl Ebs {
    fn boxed(self) -> Box<dyn Execute> {
        match self {
            Self::CompleteSnapshot(complete_snapshot) => Box::new(complete_snapshot),
            Self::StartSnapshot(start_snapshot) => Box::new(start_snapshot),
            Self::ListSnapshotBlocks(list_snapshot_blocks) => Box::new(list_snapshot_blocks),
            Self::ListChangedBlocks(list_changed_blocks) => Box::new(list_changed_blocks),
            Self::GetSnapshotBlock(get_snapshot_block) => Box::new(get_snapshot_block),
            // Self::DescribeUpdate(describe_update) => Box::new(describe_update),
            // Self::ListAddons(list_addons) => Box::new(list_addons),
            // Self::DescribeAddon(describe_addon) => Box::new(describe_addon),
        }
    }

    pub async fn dispatch(self, config: Config) -> Result<(), RawsError<ebs::Error>> {
        let client = ebs::Client::new(config.config());
        self.boxed()
            .execute(client)
            .await
            .map(|output| config.show(output))?;
        Ok(())
    }
}
