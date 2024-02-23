use super::*;

impl Show for aws_sdk_ebs::types::Block {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "INDEX\t" { self.block_index()._fmt() } "\t"
            "TOKEN\t" { self.block_token()._fmt() }
        ))
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
}

impl Show for aws_sdk_ebs::types::Status {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for aws_sdk_ebs::types::Tag {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "TAGS\t\t" { self.key()._fmt() } "\t" { self.value()._fmt() }
        ))
    }
}

impl Show for aws_sdk_ebs::types::ChecksumAlgorithm {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
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
}

impl Show for aws_sdk_ebs::operation::put_snapshot_block::PutSnapshotBlockOutput {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { self.checksum_algorithm()._fmt() } "\t" { self.checksum()._fmt() }
        ))
    }
}

impl Show for aws_sdk_ebs::operation::get_snapshot_block::GetSnapshotBlockOutput {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            { prefixed_item("DATA_LENGTH", self.data_length()) } "\t"
            { prefixed_item("CKSUM_ALGO", self.checksum_algorithm() )} "\t"
            { prefixed_item("CKSUM", self.checksum()) }
        ))
    }
}
