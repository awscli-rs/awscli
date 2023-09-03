use aws_sdk_ec2 as ec2;
use ec2::error::ProvideErrorMetadata;

use super::*;

impl AwsError for ec2::Error {
    type DisplayErrorContext<'a> = ec2::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        ec2::error::DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::Unhandled(e) => e.message(),
            _ => None,
        }
    }
}
