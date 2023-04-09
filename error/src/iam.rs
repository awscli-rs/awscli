use aws_sdk_iam as iam;
use iam::error::ProvideErrorMetadata;

use super::*;

impl AwsError for iam::Error {
    type DisplayErrorContext<'a> = iam::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        iam::error::DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
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

    fn message(&self) -> Option<&str> {
        match self {
            Self::ConcurrentModificationException(e) => e.message(),
            Self::CredentialReportExpiredException(e) => e.message(),
            Self::CredentialReportNotPresentException(e) => e.message(),
            Self::CredentialReportNotReadyException(e) => e.message(),
            Self::DeleteConflictException(e) => e.message(),
            Self::DuplicateCertificateException(e) => e.message(),
            Self::DuplicateSshPublicKeyException(e) => e.message(),
            Self::EntityAlreadyExistsException(e) => e.message(),
            Self::EntityTemporarilyUnmodifiableException(e) => e.message(),
            Self::InvalidAuthenticationCodeException(e) => e.message(),
            Self::InvalidCertificateException(e) => e.message(),
            Self::InvalidInputException(e) => e.message(),
            Self::InvalidPublicKeyException(e) => e.message(),
            Self::InvalidUserTypeException(e) => e.message(),
            Self::KeyPairMismatchException(e) => e.message(),
            Self::LimitExceededException(e) => e.message(),
            Self::MalformedCertificateException(e) => e.message(),
            Self::MalformedPolicyDocumentException(e) => e.message(),
            Self::NoSuchEntityException(e) => e.message(),
            Self::PasswordPolicyViolationException(e) => e.message(),
            Self::PolicyEvaluationException(e) => e.message(),
            Self::PolicyNotAttachableException(e) => e.message(),
            Self::ReportGenerationLimitExceededException(e) => e.message(),
            Self::ServiceFailureException(e) => e.message(),
            Self::ServiceNotSupportedException(e) => e.message(),
            Self::UnmodifiableEntityException(e) => e.message(),
            Self::UnrecognizedPublicKeyEncodingException(e) => e.message(),
            Self::Unhandled(e) => e.message(),
            _ => None,
        }
    }
}
