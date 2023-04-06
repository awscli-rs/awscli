use aws_sdk_dynamodb as dynamodb;
use dynamodb::error::ProvideErrorMetadata;

use super::*;

impl AwsError for dynamodb::Error {
    type DisplayErrorContext<'a> = dynamodb::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        dynamodb::error::DisplayErrorContext(self)
    }

    fn code(&self) -> Option<&str> {
        match self {
            Self::BackupInUseException(e) => e.code(),
            Self::BackupNotFoundException(e) => e.code(),
            Self::ConditionalCheckFailedException(e) => e.code(),
            Self::ContinuousBackupsUnavailableException(e) => e.code(),
            Self::DuplicateItemException(e) => e.code(),
            Self::ExportConflictException(e) => e.code(),
            Self::ExportNotFoundException(e) => e.code(),
            Self::GlobalTableAlreadyExistsException(e) => e.code(),
            Self::GlobalTableNotFoundException(e) => e.code(),
            Self::IdempotentParameterMismatchException(e) => e.code(),
            Self::ImportConflictException(e) => e.code(),
            Self::ImportNotFoundException(e) => e.code(),
            Self::IndexNotFoundException(e) => e.code(),
            Self::InternalServerError(e) => e.code(),
            Self::InvalidEndpointException(e) => e.code(),
            Self::InvalidExportTimeException(e) => e.code(),
            Self::InvalidRestoreTimeException(e) => e.code(),
            Self::ItemCollectionSizeLimitExceededException(e) => e.code(),
            Self::LimitExceededException(e) => e.code(),
            Self::PointInTimeRecoveryUnavailableException(e) => e.code(),
            Self::ProvisionedThroughputExceededException(e) => e.code(),
            Self::ReplicaAlreadyExistsException(e) => e.code(),
            Self::ReplicaNotFoundException(e) => e.code(),
            Self::RequestLimitExceeded(e) => e.code(),
            Self::ResourceInUseException(e) => e.code(),
            Self::ResourceNotFoundException(e) => e.code(),
            Self::TableAlreadyExistsException(e) => e.code(),
            Self::TableInUseException(e) => e.code(),
            Self::TableNotFoundException(e) => e.code(),
            Self::TransactionCanceledException(e) => e.code(),
            Self::TransactionConflictException(e) => e.code(),
            Self::TransactionInProgressException(e) => e.code(),
            Self::Unhandled(unhandled) => unhandled.code(),
            _ => None,
        }
    }

    fn message(&self) -> Option<&str> {
        match self {
            Self::BackupInUseException(e) => e.message(),
            Self::BackupNotFoundException(e) => e.message(),
            Self::ConditionalCheckFailedException(e) => e.message(),
            Self::ContinuousBackupsUnavailableException(e) => e.message(),
            Self::DuplicateItemException(e) => e.message(),
            Self::ExportConflictException(e) => e.message(),
            Self::ExportNotFoundException(e) => e.message(),
            Self::GlobalTableAlreadyExistsException(e) => e.message(),
            Self::GlobalTableNotFoundException(e) => e.message(),
            Self::IdempotentParameterMismatchException(e) => e.message(),
            Self::ImportConflictException(e) => e.message(),
            Self::ImportNotFoundException(e) => e.message(),
            Self::IndexNotFoundException(e) => e.message(),
            Self::InternalServerError(e) => e.message(),
            Self::InvalidEndpointException(e) => e.message(),
            Self::InvalidExportTimeException(e) => e.message(),
            Self::InvalidRestoreTimeException(e) => e.message(),
            Self::ItemCollectionSizeLimitExceededException(e) => e.message(),
            Self::LimitExceededException(e) => e.message(),
            Self::PointInTimeRecoveryUnavailableException(e) => e.message(),
            Self::ProvisionedThroughputExceededException(e) => e.message(),
            Self::ReplicaAlreadyExistsException(e) => e.message(),
            Self::ReplicaNotFoundException(e) => e.message(),
            Self::RequestLimitExceeded(e) => e.message(),
            Self::ResourceInUseException(e) => e.message(),
            Self::ResourceNotFoundException(e) => e.message(),
            Self::TableAlreadyExistsException(e) => e.message(),
            Self::TableInUseException(e) => e.message(),
            Self::TableNotFoundException(e) => e.message(),
            Self::TransactionCanceledException(e) => e.message(),
            Self::TransactionConflictException(e) => e.message(),
            Self::TransactionInProgressException(e) => e.message(),
            Self::Unhandled(unhandled) => unhandled.message(),
            _ => None,
        }
    }
}
