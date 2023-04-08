use miette::Diagnostic;
use thiserror::Error;

mod dynamodb;

#[derive(Debug, Error)]
#[error("RAWS CLI Error")]
pub struct RawsError<E: AwsError> {
    #[from]
    source: E,
}

pub trait AwsError: ::std::error::Error + 'static {
    type DisplayErrorContext<'a>;
    fn error_context(&self) -> Self::DisplayErrorContext<'_>;
    fn code(&self) -> Option<&str>;
    fn message(&self) -> Option<&str>;
}

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
