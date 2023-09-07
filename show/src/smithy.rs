use aws_smithy_types_convert::date_time::DateTimeExt;
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;
use time::UtcOffset;

use super::*;

impl Show for aws_smithy_types::DateTime {
    fn _fmt(&self) -> Box<dyn fmt::Display> {
        let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
        let text = self
            .to_time()
            .map_or(OffsetDateTime::UNIX_EPOCH, |ts| ts.to_offset(offset))
            .format(&Rfc3339)
            .unwrap_or_default();
        Box::new(text)
    }

    fn text(&self) -> String {
        let offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
        self.to_time()
            .map_or(OffsetDateTime::UNIX_EPOCH, |ts| ts.to_offset(offset))
            .format(&Rfc3339)
            .unwrap_or_default()
    }
}
