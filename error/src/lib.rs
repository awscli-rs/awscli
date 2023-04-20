use aws_smithy_types::error::metadata::EMPTY_ERROR_METADATA;
use aws_smithy_types::error::ErrorMetadata;
use aws_smithy_types::error::Unhandled;
use miette::Diagnostic;
use thiserror::Error;

mod dynamodb;
mod iam;
mod unhandled;

#[derive(Debug, Error)]
#[error("RAWS CLI Error")]
pub struct RawsError<E: AwsError> {
    #[from]
    source: E,
}

pub trait AwsError: ::std::error::Error + 'static + Sized {
    type DisplayErrorContext<'a>;
    fn error_context(&self) -> Self::DisplayErrorContext<'_>;

    fn error_meta(&self) -> &ErrorMetadata;

    fn code(&self) -> Option<&str> {
        self.error_meta().code()
    }

    fn message(&self) -> Option<&str> {
        self.error_meta().message()
    }

    fn try_into_unhandled(self) -> Result<Unhandled, Self>;
}

// impl<E: AwsError> From<E> for RawsError<E> {
//     fn from(e: E) -> Self {
//         match e.try_into_unhandled() {
//             Ok(unhandled) => Self { source: unhandled },
//             Err(other) => Self { source: other },
//         }
//     }
// }

impl<E: AwsError> Diagnostic for RawsError<E> {
    fn code<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self.source.code() {
            Some(code) => Some(Box::new(code)),
            None => None,
        }
    }

    fn help<'a>(&'a self) -> Option<Box<dyn std::fmt::Display + 'a>> {
        match self.source.message() {
            Some(code) => Some(Box::new(code)),
            None => None,
        }
    }
}
