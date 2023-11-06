use aws_sdk_s3 as s3;

use super::*;

impl AwsError for s3::Error {
    type DisplayErrorContext<'a> = DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::BucketAlreadyExists(e) => e.meta(),
            Self::BucketAlreadyOwnedByYou(e) => e.meta(),
            Self::InvalidObjectState(e) => e.meta(),
            Self::NoSuchBucket(e) => e.meta(),
            Self::NoSuchKey(e) => e.meta(),
            Self::NoSuchUpload(e) => e.meta(),
            Self::NotFound(e) => e.meta(),
            Self::ObjectAlreadyInActiveTierError(e) => e.meta(),
            Self::ObjectNotInActiveTierError(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }
}
