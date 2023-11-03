use std::str;

use super::*;

pub fn parse_tags(text: &str) -> Result<Vec<TagSpecification>, BuildError> {
    let (resources, tags) = text.split_once(',').ok_or_else(missing_comma)?;

    let tags = tags
        .strip_prefix("Tags=[{")
        .ok_or_else(missing_tag)?
        .strip_suffix("}]")
        .ok_or_else(missing_tag)?
        .split("},{")
        .map(tag)
        .collect::<Result<Vec<_>, _>>()?;

    let tags = resources
        .strip_prefix("ResourceType=")
        .ok_or_else(missing_resource)?
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

fn tag(text: &str) -> Result<Tag, BuildError> {
    crate::tag::parse_tag(text)
}

fn missing_comma() -> BuildError {
    BuildError::other("Tag specification should have comma-separated resource type and tags")
}

fn missing_tag() -> BuildError {
    BuildError::missing_field("Tag=", "Tag should have'Tag=[{{...}}]' element")
}

fn missing_resource() -> BuildError {
    BuildError::missing_field(
        "ResourceType=",
        "Tag should have 'ResourceType=xxx' element",
    )
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
