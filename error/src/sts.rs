use aws_sdk_sts as sts;

use super::*;

impl AwsError for sts::Error {
    type DisplayErrorContext<'a> = DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        DisplayErrorContext(self)
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
}
