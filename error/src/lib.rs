use miette::Diagnostic;
use thiserror::Error;

pub type Result<T, E> = ::std::result::Result<T, Error<E>>;

#[derive(Debug, Diagnostic, Error)]
#[diagnostic(help = "AWS SDK error")]
#[error("RAWS Error")]
pub struct Error<E: AwsError> {
    #[from]
    source: E,
}

pub trait AwsError: ::std::error::Error + 'static {}

impl AwsError for aws_sdk_dynamodb::Error {}
