use aws_smithy_types::date_time::DateTimeParseError;
use aws_smithy_types::date_time::Format;
use aws_smithy_types::DateTime;

pub fn datetime_parser(text: &str) -> Result<DateTime, DateTimeParseError> {
    DateTime::from_str(text, Format::DateTimeWithOffset)
}
