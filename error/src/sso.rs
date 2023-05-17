use aws_sdk_sso as sso;
use sso::error::ProvideErrorMetadata;

use super::*;

impl AwsError for sso::Error {
    type DisplayErrorContext<'a> = sso::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        sso::error::DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::InvalidRequestException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::TooManyRequestsException(e) => e.meta(),
            Self::UnauthorizedException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::InvalidRequestException(e) => e.message(),
            Self::ResourceNotFoundException(e) => e.message(),
            Self::TooManyRequestsException(e) => e.message(),
            Self::UnauthorizedException(e) => e.message(),
            Self::Unhandled(e) => e.message(),
            _ => None,
        }
    }
}
