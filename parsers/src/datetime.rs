use aws_smithy_types::DateTime;
use aws_smithy_types_convert::date_time::DateTimeExt;
use time::format_description::well_known::Rfc3339;

pub fn datetime_parser(text: &str) -> Result<DateTime, time::error::Parse> {
    time::OffsetDateTime::parse(text, &Rfc3339).map(DateTime::from_time)
}
