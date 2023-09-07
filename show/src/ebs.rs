use super::*;

impl Show for aws_sdk_ebs::types::Block {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "INDEX\t" { self.block_index()._fmt() } "\t"
            "TOKEN\t" { self.block_token()._fmt() }
        ))
    }

    fn text(&self) -> String {
        let index = self.block_index().unwrap_or_default();
        let token = self.block_token().unwrap_or_default();

        fmtools::format!(
            "INDEX\t" {index} "\tTOKEN\t" {token}
        )
    }
}

impl Show for aws_sdk_ebs::types::ChangedBlock {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "INDEX\t" { self.block_index()._fmt() } "\n"
            "\tFIRST_TOKEN\t" { self.first_block_token()._fmt() } "\n"
            "\tSECOND_TOKEN\t" { self.second_block_token()._fmt() }

        ))
    }

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
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for aws_sdk_ebs::types::Tag {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "TAGS\t\t" { self.key()._fmt() } "\t" { self.value()._fmt() }
        ))
    }

    fn text(&self) -> String {
        let key = self.key().unwrap_or_default();
        let value = self.value().unwrap_or_default();
        fmtools::format!(
            "TAGS\t\t" {key} "\t" {value}
        )
    }
}

impl Show for aws_sdk_ebs::types::ChecksumAlgorithm {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        todo!()
    }
}

impl Show for aws_sdk_ebs::operation::start_snapshot::StartSnapshotOutput {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("DESCRIPTION", self.description()) } "\n"
            { prefixed_item("SNAPSHOT_ID", self.snapshot_id()) } "\n"
            { prefixed_item("OWNER_ID", self.owner_id()) } "\n"
            { prefixed_item("STATUS", self.status()) } "\n"
            { prefixed_item("START_TIME", self.start_time()) } "\n"
            { prefixed_item("VOLUME_SIZE", self.volume_size()) } "\n"
            { prefixed_item("BLOCK_SIZE", self.block_size()) } "\n"
            { prefixed_items("TAGS", self.tags()) } "\n"
            { prefixed_item("PARENT_SNAPSHOT_ID", self.parent_snapshot_id()) } "\n"
            { prefixed_item("KSM_KEY_ARN", self.kms_key_arn()) } "\n"
        ))
    }

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

impl Show for aws_sdk_ebs::operation::put_snapshot_block::PutSnapshotBlockOutput {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.checksum_algorithm()._fmt() } "\t" { self.checksum()._fmt() }
        ))
    }

    fn text(&self) -> String {
        let checksum = self.checksum().unwrap_or_default();
        let checksum_algo = self
            .checksum_algorithm()
            .map(|algo| algo.as_str())
            .unwrap_or_default();

        fmtools::format!(
            {checksum_algo} "\t" {checksum}
        )
    }
}

impl Show
    for (
        Option<String>,
        Option<aws_sdk_ebs::types::ChecksumAlgorithm>,
        Option<i32>,
    )
{
    fn _fmt(&self) -> Box<dyn fmt::Display> {
        todo!()
    }

    fn text(&self) -> String {
        todo!()
    }
}
