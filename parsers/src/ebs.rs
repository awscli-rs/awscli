use aws_sdk_ebs::types::Tag;

use super::*;

pub fn parse_tags(text: &str) -> Result<Vec<Tag>, InvalidTag> {
    text.split_whitespace()
        .map(parse_tag)
        .inspect(|tag| println!("{tag:?}"))
        .collect()
}

pub fn parse_tag(text: &str) -> Result<Tag, InvalidTag> {
    text.split_once(',')
        .ok_or(InvalidTag::MissingComma)
        .and_then(tag)
}

fn tag((key, value): (&str, &str)) -> Result<Tag, InvalidTag> {
    let key = parse_key(key)?;
    let value = parse_value(value)?;
    Ok(Tag::builder().key(key).value(value).build())
}

fn parse_key(text: &str) -> Result<&str, InvalidTag> {
    text.strip_prefix("Key=").ok_or(InvalidTag::MissingKey)
}

fn parse_value(text: &str) -> Result<&str, InvalidTag> {
    text.strip_prefix("Value=").ok_or(InvalidTag::MissingValue)
}

#[derive(Debug, Error, PartialEq)]
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

    #[test]
    fn single_tag() {
        let tags = parse_tags("Key=k1,Value=v1").unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].key().unwrap(), "k1");
        assert_eq!(tags[0].value().unwrap(), "v1");
    }

    #[test]
    fn multiple_tags() {
        let tags = parse_tags("Key=k1,Value=v1 Key=k2,Value=v2").unwrap();
        assert_eq!(tags.len(), 2);
        assert_eq!(tags[0].key().unwrap(), "k1");
        assert_eq!(tags[0].value().unwrap(), "v1");
        assert_eq!(tags[1].key().unwrap(), "k2");
        assert_eq!(tags[1].value().unwrap(), "v2");
    }

    #[test]
    fn no_comma() {
        let e = parse_tags("Key=k-Value=v").unwrap_err();
        assert_eq!(e, InvalidTag::MissingComma);
    }

    #[test]
    fn missing_key() {
        let e = parse_tags("XKey=k,Value=v").unwrap_err();
        assert_eq!(e, InvalidTag::MissingKey);
    }

    #[test]
    fn missing_value() {
        let e = parse_tags("Key=k,XValue=v").unwrap_err();
        assert_eq!(e, InvalidTag::MissingValue);
    }

    #[test]
    fn empty_key() {
        let tags = parse_tags("Key=,Value=v").unwrap();
        assert_eq!(tags[0].key().unwrap(), "");
    }

    #[test]
    fn empty_value() {
        let tags = parse_tags("Key=k,Value=").unwrap();
        assert_eq!(tags[0].value().unwrap(), "");
    }
}
