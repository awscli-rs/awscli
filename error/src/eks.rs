use aws_sdk_eks as eks;

use super::*;

impl AwsError for eks::Error {
    type DisplayErrorContext<'a> = DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        DisplayErrorContext(self)
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
}
