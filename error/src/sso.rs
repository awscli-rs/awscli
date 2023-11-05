use aws_sdk_sso as sso;

use super::*;

impl AwsError for sso::Error {
    type DisplayErrorContext<'a> = DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        DisplayErrorContext(self)
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
}
