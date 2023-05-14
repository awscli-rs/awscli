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
            sso::Error::InvalidRequestException(e) => e.meta(),
            sso::Error::ResourceNotFoundException(e) => e.meta(),
            sso::Error::TooManyRequestsException(e) => e.meta(),
            sso::Error::UnauthorizedException(e) => e.meta(),
            sso::Error::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }

    fn message(&self) -> Option<&str> {
        match self {
            sso::Error::InvalidRequestException(e) => e.message(),
            sso::Error::ResourceNotFoundException(e) => e.message(),
            sso::Error::TooManyRequestsException(e) => e.message(),
            sso::Error::UnauthorizedException(e) => e.message(),
            sso::Error::Unhandled(e) => e.message(),
            _ => None,
        }
    }
}
