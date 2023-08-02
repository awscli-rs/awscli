use aws_smithy_types_convert::date_time::DateTimeExt;
use time::format_description::well_known::Rfc3339;

use super::*;

impl Show for aws_smithy_types::DateTime {
    fn text(&self) -> String {
        let offset = time::UtcOffset::current_local_offset().unwrap_or(time::UtcOffset::UTC);
        self.to_time()
            .map(|ts| ts.to_offset(offset))
            .unwrap_or(time::OffsetDateTime::UNIX_EPOCH)
            .format(&Rfc3339)
            .unwrap_or_default()
    }
}
