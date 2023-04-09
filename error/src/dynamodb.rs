use aws_sdk_dynamodb as dynamodb;
use dynamodb::error::ProvideErrorMetadata;

use super::*;

impl AwsError for dynamodb::Error {
    type DisplayErrorContext<'a> = dynamodb::error::DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        dynamodb::error::DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        match self {
            Self::BackupInUseException(e) => e.meta(),
            Self::BackupNotFoundException(e) => e.meta(),
            Self::ConditionalCheckFailedException(e) => e.meta(),
            Self::ContinuousBackupsUnavailableException(e) => e.meta(),
            Self::DuplicateItemException(e) => e.meta(),
            Self::ExportConflictException(e) => e.meta(),
            Self::ExportNotFoundException(e) => e.meta(),
            Self::GlobalTableAlreadyExistsException(e) => e.meta(),
            Self::GlobalTableNotFoundException(e) => e.meta(),
            Self::IdempotentParameterMismatchException(e) => e.meta(),
            Self::ImportConflictException(e) => e.meta(),
            Self::ImportNotFoundException(e) => e.meta(),
            Self::IndexNotFoundException(e) => e.meta(),
            Self::InternalServerError(e) => e.meta(),
            Self::InvalidEndpointException(e) => e.meta(),
            Self::InvalidExportTimeException(e) => e.meta(),
            Self::InvalidRestoreTimeException(e) => e.meta(),
            Self::ItemCollectionSizeLimitExceededException(e) => e.meta(),
            Self::LimitExceededException(e) => e.meta(),
            Self::PointInTimeRecoveryUnavailableException(e) => e.meta(),
            Self::ProvisionedThroughputExceededException(e) => e.meta(),
            Self::ReplicaAlreadyExistsException(e) => e.meta(),
            Self::ReplicaNotFoundException(e) => e.meta(),
            Self::RequestLimitExceeded(e) => e.meta(),
            Self::ResourceInUseException(e) => e.meta(),
            Self::ResourceNotFoundException(e) => e.meta(),
            Self::TableAlreadyExistsException(e) => e.meta(),
            Self::TableInUseException(e) => e.meta(),
            Self::TableNotFoundException(e) => e.meta(),
            Self::TransactionCanceledException(e) => e.meta(),
            Self::TransactionConflictException(e) => e.meta(),
            Self::TransactionInProgressException(e) => e.meta(),
            Self::Unhandled(e) => e.meta(),
            _ => &EMPTY_ERROR_METADATA,
        }
    }
}
