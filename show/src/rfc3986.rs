use super::*;

#[derive(Debug)]
pub(crate) struct Rfc3986<'a>(pub(crate) &'a str);

impl Show for Rfc3986<'_> {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(percent_encoding::percent_decode_str(self.0).decode_utf8_lossy())
    }
}
