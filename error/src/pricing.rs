use aws_sdk_pricing as pricing;

use super::*;

impl AwsError for pricing::Error {
    type DisplayErrorContext<'a> = DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::AccessDeniedException(e) => e.meta(),
            Self::ExpiredNextTokenException(e) => e.meta(),
            Self::InternalErrorException(e) => e.meta(),
            Self::InvalidNextTokenException(e) => e.meta(),
            Self::InvalidParameterException(e) => e.meta(),
            Self::NotFoundException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }
}
