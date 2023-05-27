use aws_sdk_eks as eks;
use eks::error::ProvideErrorMetadata;

use super::*;

impl AwsError for eks::Error {
    type DisplayErrorContext<'a> = eks::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        eks::error::DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::AccessDeniedException(e) => e.meta(),
            Self::BadRequestException(e) => e.meta(),
            Self::ClientException(e) => e.meta(),
            Self::InvalidParameterException(e) => e.meta(),
            Self::InvalidRequestException(e) => e.meta(),
            Self::NotFoundException(e) => e.meta(),
            Self::ResourceInUseException(e) => e.meta(),
            Self::ResourceLimitExceededException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::ResourcePropagationDelayException(e) => e.meta(),
            Self::ServerException(e) => e.meta(),
            Self::ServiceUnavailableException(e) => e.meta(),
            Self::UnsupportedAvailabilityZoneException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.message(),
            Self::BadRequestException(e) => e.message(),
            Self::ClientException(e) => e.message(),
            Self::InvalidParameterException(e) => e.message(),
            Self::InvalidRequestException(e) => e.message(),
            Self::NotFoundException(e) => e.message(),
            Self::ResourceInUseException(e) => e.message(),
            Self::ResourceLimitExceededException(e) => e.message(),
            Self::ResourceNotFoundException(e) => e.message(),
            Self::ResourcePropagationDelayException(e) => e.message(),
            Self::ServerException(e) => e.message(),
            Self::ServiceUnavailableException(e) => e.message(),
            Self::UnsupportedAvailabilityZoneException(e) => e.message(),
            Self::Unhandled(e) => e.message(),
            _ => None,
        }
    }
}
