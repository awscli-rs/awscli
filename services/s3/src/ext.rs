use std::ops::Not;

use super::*;

pub(crate) trait TextExt: Sized {
    fn none_if_empty(self) -> Option<Self>;
}

impl TextExt for &str {
    fn none_if_empty(self) -> Option<Self> {
        self.is_empty().not().then_some(self)
    }
}

pub(crate) fn maybe_bucket_name(s3_url: Option<&str>) -> Option<&str> {
    s3_url?
        .none_if_empty()?
        .trim_start_matches(S3_PREFIX)
        .none_if_empty()
}

pub(crate) fn bucket_name(arg: &str) -> &str {
    arg.trim_start_matches(S3_PREFIX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn none() {
        let name = maybe_bucket_name(None);
        assert!(name.is_none());
    }

    #[test]
    fn prefix_only() {
        let name = maybe_bucket_name(Some("s3://"));
        assert!(name.is_none());
    }

    #[test]
    fn with_prefix() {
        let name = maybe_bucket_name(Some("s3://bucket-name")).unwrap();
        assert_eq!(name, "bucket-name");
    }

    #[test]
    fn no_prefix() {
        let name = maybe_bucket_name(Some("bucket-name")).unwrap();
        assert_eq!(name, "bucket-name");
    }

    #[test]
    fn empty() {
        let name = maybe_bucket_name(Some(""));
        assert!(name.is_none());
    }
}
