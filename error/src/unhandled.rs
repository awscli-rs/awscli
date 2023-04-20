use std::error::Error;

use aws_sdk_dynamodb::error::ProvideErrorMetadata;
use aws_smithy_types::error::display::DisplayErrorContext;

use super::*;

impl AwsError for Unhandled {
    type DisplayErrorContext<'a> = DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        DisplayErrorContext(self)
    }

    fn error_meta(&self) -> &ErrorMetadata {
        self.meta()
    }

    fn try_into_unhandled(self) -> Result<Unhandled, Self> {
        Err(self)
    }
}
