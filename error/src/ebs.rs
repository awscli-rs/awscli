use aws_sdk_ebs as ebs;
use ebs::error::ProvideErrorMetadata;

use super::*;

impl AwsError for ebs::Error {
    type DisplayErrorContext<'a> = ebs::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        ebs::error::DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::AccessDeniedException(e) => e.meta(),
            Self::ConcurrentLimitExceededException(e) => e.meta(),
            Self::ConflictException(e) => e.meta(),
            Self::InternalServerException(e) => e.meta(),
            Self::RequestThrottledException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::ServiceQuotaExceededException(e) => e.meta(),
            Self::ValidationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.message(),
            Self::ConcurrentLimitExceededException(e) => e.message(),
            Self::ConflictException(e) => e.message(),
            Self::InternalServerException(e) => e.message(),
            Self::RequestThrottledException(e) => e.message(),
            Self::ResourceNotFoundException(e) => e.message(),
            Self::ServiceQuotaExceededException(e) => e.message(),
            Self::ValidationException(e) => e.message(),
            Self::Unhandled(e) => e.message(),
            _ => None,
        }
    }
}