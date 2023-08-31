use aws_sdk_sts::types::PolicyDescriptorType;

use super::*;

pub fn parse_arns(text: &str) -> Result<Vec<PolicyDescriptorType>, InvalidPolicyDescriptorType> {
    text.split_whitespace()
        .map(parse_arn)
        // .inspect(|tag| println!("{tag:?}"))
        .collect()
}

fn parse_arn(text: &str) -> Result<PolicyDescriptorType, InvalidPolicyDescriptorType> {
    text.strip_prefix("arn=")
        .ok_or(InvalidPolicyDescriptorType::MissingArn)
        .map(|input| PolicyDescriptorType::builder().arn(input).build())
}

#[derive(Clone, Debug, PartialEq, Error)]
pub enum InvalidPolicyDescriptorType {
    #[error("Missing 'arn=' prefix")]
    MissingArn,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_arn() {
        let arns = parse_arns("arn=text1").unwrap();
        assert_eq!(arns.len(), 1);
        assert_eq!(arns[0].arn().unwrap(), "text1");
    }

    #[test]
    fn multiple_tags() {
        let arns = parse_arns("arn=text1 arn=text2").unwrap();
        assert_eq!(arns.len(), 2);
        assert_eq!(arns[0].arn().unwrap(), "text1");
        assert_eq!(arns[1].arn().unwrap(), "text2");
    }

    #[test]
    fn invalid_arn_prefix() {
        let e = parse_arns("sdfsfdfs").unwrap_err();
        assert_eq!(e, InvalidPolicyDescriptorType::MissingArn);
    }
}
