use aws_sdk_account as account;

use account::error::ProvideErrorMetadata;

use super::*;

impl AwsError for account::Error {
    type DisplayErrorContext<'a> = account::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        account::error::DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::AccessDeniedException(e) => e.meta(),
            Self::ConflictException(e) => e.meta(),
            Self::InternalServerException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::TooManyRequestsException(e) => e.meta(),
            Self::ValidationException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.message(),
            Self::ConflictException(e) => e.message(),
            Self::InternalServerException(e) => e.message(),
            Self::ResourceNotFoundException(e) => e.message(),
            Self::TooManyRequestsException(e) => e.message(),
            Self::ValidationException(e) => e.message(),
            Self::Unhandled(e) => e.message(),
            _ => None,
        }
    }
}
