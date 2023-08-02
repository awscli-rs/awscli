use super::*;

impl Show for aws_sdk_ebs::types::Status {
    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for aws_sdk_ebs::operation::start_snapshot::StartSnapshotOutput {
    fn text(&self) -> String {
        let description = self.description().unwrap_or_default();
        let snapshot_id = self.snapshot_id().unwrap_or_default();
        let owner_id = self.owner_id().unwrap_or_default();
        let status = self.status().text();
        let start_time = self.start_time();
        let volume_size = self.volume_size().unwrap_or_default();
        let block_size = self.block_size().unwrap_or_default();
        let tags = self.tags().unwrap_or_default();
        let parent_snapshot_id = self.parent_snapshot_id();
        let kms_key_arn = self.kms_key_arn();

        fmtools::format!(
            "DESCRIPTION\t" {description}
            "SNAPSHOT_ID\t" {snapshot_id}
            "OWNER_ID\t"    {owner_id}
            "STATUS\t"      {status}
            if let Some(start_time) = start_time {
                "START_TIME\t" {start_time:?}
            }
            "VOLUME_SIZE\t" {volume_size}
            "BLOCK_SIZE\t"  {block_size}
            "TAGS\t"        {tags:?}
            if let Some(parent_snapshot_id) = parent_snapshot_id {
                "PARENT_SNAPSHOT_ID\t" {parent_snapshot_id}
            }
            if let Some(kms_key_arn) = kms_key_arn {
                "KSM_KEY_ARN\t" {kms_key_arn}
            }
        )
    }
}
