use aws_sdk_sts as sts;
use sts::error::ProvideErrorMetadata;

use super::*;

impl AwsError for sts::Error {
    type DisplayErrorContext<'a> = sts::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        sts::error::DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::ExpiredTokenException(e) => e.meta(),
            Self::IdpCommunicationErrorException(e) => e.meta(),
            Self::IdpRejectedClaimException(e) => e.meta(),
            Self::InvalidAuthorizationMessageException(e) => e.meta(),
            Self::InvalidIdentityTokenException(e) => e.meta(),
            Self::MalformedPolicyDocumentException(e) => e.meta(),
            Self::PackedPolicyTooLargeException(e) => e.meta(),
            Self::RegionDisabledException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::ExpiredTokenException(e) => e.message(),
            Self::IdpCommunicationErrorException(e) => e.message(),
            Self::IdpRejectedClaimException(e) => e.message(),
            Self::InvalidAuthorizationMessageException(e) => e.message(),
            Self::InvalidIdentityTokenException(e) => e.message(),
            Self::MalformedPolicyDocumentException(e) => e.message(),
            Self::PackedPolicyTooLargeException(e) => e.message(),
            Self::RegionDisabledException(e) => e.message(),
            Self::Unhandled(e) => e.message(),
            _ => None,
        }
    }
}
