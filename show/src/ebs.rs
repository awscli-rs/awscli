use super::*;

impl Show for aws_sdk_ebs::types::Block {
    fn text(&self) -> String {
        let index = self.block_index().unwrap_or_default();
        let token = self.block_token().unwrap_or_default();

        fmtools::format!(
            "INDEX\t" {index} "\tTOKEN\t" {token}
        )
    }
}

impl Show for aws_sdk_ebs::types::ChangedBlock {
    fn text(&self) -> String {
        let index = self.block_index().unwrap_or_default();
        let first_token = self.first_block_token().unwrap_or_default();
        let second_token = self.second_block_token().unwrap_or_default();

        fmtools::format!(
            "INDEX\t" {index}
            "\tFIRST_TOKEN\t" {first_token}
            "\tSECOND_TOKEN\t" {second_token}
        )
    }
}

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
        let start_time = self.start_time().text();
        let volume_size = self.volume_size().unwrap_or_default();
        let block_size = self.block_size().unwrap_or_default();
        let tags = self.tags().unwrap_or_default();
        let parent_snapshot_id = self.parent_snapshot_id();
        let kms_key_arn = self.kms_key_arn();

        fmtools::format!(
            "DESCRIPTION\t" {description} "\n"
            "SNAPSHOT_ID\t" {snapshot_id} "\n"
            "OWNER_ID\t"    {owner_id} "\n"
            "STATUS\t\t"    {status} "\n"
            "START_TIME\t"  {start_time} "\n"
            "VOLUME_SIZE\t" {volume_size} "\n"
            "BLOCK_SIZE\t"  {block_size} "\n"

            for tag in tags {
                {tag.text()} "\n"
            }

            if let Some(parent_snapshot_id) = parent_snapshot_id {
                "PARENT_SNAPSHOT_ID\t" {parent_snapshot_id} "\n"
            }
            if let Some(kms_key_arn) = kms_key_arn {
                "KSM_KEY_ARN\t" {kms_key_arn} "\n"
            }
        )
    }
}

impl Show for aws_sdk_ebs::types::Tag {
    fn text(&self) -> String {
        let key = self.key().unwrap_or_default();
        let value = self.value().unwrap_or_default();
        fmtools::format!(
            "TAGS\t\t" {key} "\t" {value}
        )
    }
}