use aws_sdk_iam as iam;
use iam::error::ProvideErrorMetadata;

use super::*;

impl AwsError for iam::Error {
    type DisplayErrorContext<'a> = iam::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        iam::error::DisplayErrorContext(self)
    }

    fn error_meta(&self) -> &ErrorMetadata {
        match self {
            Self::ConcurrentModificationException(e) => e.meta(),
            Self::CredentialReportExpiredException(e) => e.meta(),
            Self::CredentialReportNotPresentException(e) => e.meta(),
            Self::CredentialReportNotReadyException(e) => e.meta(),
            Self::DeleteConflictException(e) => e.meta(),
            Self::DuplicateCertificateException(e) => e.meta(),
            Self::DuplicateSshPublicKeyException(e) => e.meta(),
            Self::EntityAlreadyExistsException(e) => e.meta(),
            Self::EntityTemporarilyUnmodifiableException(e) => e.meta(),
            Self::InvalidAuthenticationCodeException(e) => e.meta(),
            Self::InvalidCertificateException(e) => e.meta(),
            Self::InvalidInputException(e) => e.meta(),
            Self::InvalidPublicKeyException(e) => e.meta(),
            Self::InvalidUserTypeException(e) => e.meta(),
            Self::KeyPairMismatchException(e) => e.meta(),
            Self::LimitExceededException(e) => e.meta(),
            Self::MalformedCertificateException(e) => e.meta(),
            Self::MalformedPolicyDocumentException(e) => e.meta(),
            Self::NoSuchEntityException(e) => e.meta(),
            Self::PasswordPolicyViolationException(e) => e.meta(),
            Self::PolicyEvaluationException(e) => e.meta(),
            Self::PolicyNotAttachableException(e) => e.meta(),
            Self::ReportGenerationLimitExceededException(e) => e.meta(),
            Self::ServiceFailureException(e) => e.meta(),
            Self::ServiceNotSupportedException(e) => e.meta(),
            Self::UnmodifiableEntityException(e) => e.meta(),
            Self::UnrecognizedPublicKeyEncodingException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }

    fn try_into_unhandled(self) -> Result<Unhandled, Self> {
        match self {
            Self::Unhandled(unhandled) => Ok(unhandled),
            other => Err(other),
        }
    }
}
