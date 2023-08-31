use super::*;

pub trait Tag: std::fmt::Debug {
    fn from_key_and_value(key: &str, value: &str) -> Self;
}

impl Tag for aws_sdk_ebs::types::Tag {
    fn from_key_and_value(key: &str, value: &str) -> Self {
        Self::builder().key(key).value(value).build()
    }
}

impl Tag for aws_sdk_sts::types::Tag {
    fn from_key_and_value(key: &str, value: &str) -> Self {
        Self::builder().key(key).value(value).build()
    }
}

pub fn parse_tags<T: Tag>(text: &str) -> Result<Vec<T>, InvalidTag> {
    text.split_whitespace()
        .map(parse_tag)
        // .inspect(|tag| println!("{tag:?}"))
        .collect()
}

fn parse_tag<T: Tag>(text: &str) -> Result<T, InvalidTag> {
    text.split_once(',')
        .ok_or(InvalidTag::MissingComma)
        .and_then(tag)
}

fn tag<T: Tag>((key, value): (&str, &str)) -> Result<T, InvalidTag> {
    let key = parse_key(key)?;
    let value = parse_value(value)?;
    Ok(T::from_key_and_value(key, value))
}

fn parse_key(text: &str) -> Result<&str, InvalidTag> {
    text.strip_prefix("Key=").ok_or(InvalidTag::MissingKey)
}

fn parse_value(text: &str) -> Result<&str, InvalidTag> {
    text.strip_prefix("Value=").ok_or(InvalidTag::MissingValue)
}

#[derive(Debug, PartialEq, Error)]
pub enum InvalidTag {
    #[error("Tag should have comma separated key and value ('Key=k,Value=v')")]
    MissingComma,
    #[error("Tag should have 'Key=xxx' element")]
    MissingKey,
    #[error("Tag should have 'Value=xxx' element")]
    MissingValue,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct TestTag {
        key: String,
        value: String,
    }

    impl TestTag {
        fn key(&self) -> &str {
            &self.key
        }

        fn value(&self) -> &str {
            &self.value
        }
    }

    impl Tag for TestTag {
        fn from_key_and_value(key: &str, value: &str) -> Self {
            Self {
                key: key.to_string(),
                value: value.to_string(),
            }
        }
    }

    #[test]
    fn single_tag() {
        let tags = parse_tags::<TestTag>("Key=k1,Value=v1").unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].key(), "k1");
        assert_eq!(tags[0].value(), "v1");
    }

    #[test]
    fn multiple_tags() {
        let tags = parse_tags::<TestTag>("Key=k1,Value=v1 Key=k2,Value=v2").unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].key(), "k1");
        assert_eq!(tags[0].value(), "v1");
        assert_eq!(tags[1].key(), "k2");
        assert_eq!(tags[1].value(), "v2");
    }

    #[test]
    fn no_comma() {
        let e = parse_tags::<TestTag>("Key=k-Value=v").unwrap_err();
        assert_eq!(e, InvalidTag::MissingComma);
    }

    #[test]
    fn missing_key() {
        let e = parse_tags::<TestTag>("XKey=k,Value=v").unwrap_err();
        assert_eq!(e, InvalidTag::MissingKey);
    }

    #[test]
    fn missing_value() {
        let e = parse_tags::<TestTag>("Key=k,XValue=v").unwrap_err();
        assert_eq!(e, InvalidTag::MissingValue);
    }

    #[test]
    fn empty_key() {
        let tags = parse_tags::<TestTag>("Key=,Value=v").unwrap();
        assert_eq!(tags[0].key(), "");
    }

    #[test]
    fn empty_value() {
        let tags = parse_tags::<TestTag>("Key=k,Value=").unwrap();
        assert_eq!(tags[0].value(), "");
    }
}
