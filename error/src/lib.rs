use std::error::Error as StdError;

use aws_smithy_types::error::display::DisplayErrorContext;
use aws_smithy_types::error::metadata::ProvideErrorMetadata;
use aws_smithy_types::error::ErrorMetadata;
use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Error)]
#[error("RAWS CLI Error")]
pub struct RawsError<E: AwsError> {
    #[from]
    source: E,
}

pub trait AwsError: ::std::error::Error + 'static {
    type DisplayErrorContext<'a>;
    fn error_context(&self) -> Self::DisplayErrorContext<'_>;

    fn meta(&self) -> &ErrorMetadata;

    fn code(&self) -> Option<&str> {
        self.meta().code()
    }

    fn message(&self) -> Option<&str> {
        self.meta().message()
    }
}

impl<E: AwsError> Diagnostic for RawsError<E> {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self.source.code() {
            Some(text) => Some(Box::new(text)),
            None => None,
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self.source.message() {
            Some(text) => Some(Box::new(text)),
            None => None,
        }
    }
}

impl<T: ProvideErrorMetadata + StdError + 'static> AwsError for T {
    type DisplayErrorContext<'a> = DisplayErrorContext<&'a Self>;

    fn error_context(&self) -> Self::DisplayErrorContext<'_> {
        DisplayErrorContext(self)
    }

    fn meta(&self) -> &ErrorMetadata {
        self.meta()
    }
}
