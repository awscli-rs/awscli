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
}
