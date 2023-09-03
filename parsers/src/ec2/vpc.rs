use std::str;

use super::*;

pub fn parse_tags(text: &str) -> Result<Vec<TagSpecification>, InvalidTag> {
    let (resources, tags) = text.split_once(',').ok_or(InvalidTag::MissingComma)?;

    let tags = tags
        .strip_prefix("Tags=[{")
        .ok_or(InvalidTag::MissingTag)?
        .strip_suffix("}]")
        .ok_or(InvalidTag::MissingTag)?
        .split("},{")
        .map(tag)
        .collect::<Result<Vec<_>, _>>()?;

    let tags = resources
        .strip_prefix("ResourceType=")
        .ok_or(InvalidTag::MissingResource)?
        .split('|')
        .map(ResourceType::from)
        .map(|r| {
            TagSpecification::builder()
                .resource_type(r)
                .set_tags(Some(tags.clone()))
                .build()
        })
        .collect::<Vec<_>>();

    Ok(tags)
}

// struct ResourceIterator {}
fn tag(text: &str) -> Result<Tag, InvalidTag> {
    crate::tag::parse_tag(text).map_err(InvalidTag::from)
}

#[derive(Debug, PartialEq, Error)]
pub enum InvalidTag {
    #[error("Tag specification should have comma-separated resource type and tags")]
    MissingComma,
    #[error("Tag should have 'ResourceType=xxx' element")]
    MissingResource,
    #[error("Tag should have'Tag=[{{...}}]' element")]
    MissingTag,
    #[error("Tag should have comma separated key and value ('Key=k,Value=v')")]
    MissingKeyValueComma,
    #[error("Tag should have 'Key=xxx' element")]
    MissingKey,
    #[error("Tag should have 'Value=xxx' element")]
    MissingValue,
}

impl From<crate::tag::InvalidTag> for InvalidTag {
    fn from(value: crate::tag::InvalidTag) -> Self {
        match value {
            tag::InvalidTag::MissingComma => Self::MissingKeyValueComma,
            tag::InvalidTag::MissingKey => Self::MissingKey,
            tag::InvalidTag::MissingValue => Self::MissingValue,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut tags = parse_tags("ResourceType=vpc,Tags=[{Key=Name,Value=MyVpc}]").unwrap();
        assert_eq!(tags.len(), 1);

        let TagSpecification {
            resource_type,
            tags,
            ..
        } = tags.remove(0);
        assert_eq!(resource_type, Some(ResourceType::Vpc));

        let tags = tags.unwrap();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].key().unwrap(), "Name");
        assert_eq!(tags[0].value().unwrap(), "MyVpc");
    }

    #[test]
    fn two_tags() {
        let mut tags = parse_tags(r#"ResourceType=vpc,Tags=[{Key=Environment,Value="Preprod"},{Key=Owner,Value="Build Team"}]"#).unwrap();

        let TagSpecification {
            resource_type,
            tags,
            ..
        } = tags.remove(0);

        assert_eq!(resource_type, Some(ResourceType::Vpc));

        let mut tags = tags.unwrap();
        let tag1 = tags.remove(1);
        let tag0 = tags.remove(0);

        assert_eq!(tag0.key().unwrap(), "Environment");
        assert_eq!(tag0.value().unwrap(), "Preprod");

        assert_eq!(tag1.key().unwrap(), "Owner");
        assert_eq!(tag1.value().unwrap(), "Build Team");
    }
}
