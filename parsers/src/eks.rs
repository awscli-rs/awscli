use aws_sdk_eks::types::VpcConfigRequest;

use super::*;

pub fn vpc_config_request(text: &str) -> Result<VpcConfigRequest, InvalidVpcConfigRequest> {
    let mut vcr = VpcConfigRequest::builder();
    let items = text.split(',');

    let mut key = "";
    let mut items = items
        .map(|item| {
            if let Some((k, v)) = item.split_once('=') {
                key = k;
                (key, v)
            } else {
                (key, item)
            }
        })
        .into_group_map();

    if let Some(subnet_ids) = items.remove("subnetIds") {
        vcr = subnet_ids
            .into_iter()
            .fold(vcr, |vcr, id| vcr.subnet_ids(id));
    }

    if let Some(security_group_ids) = items.remove("securityGroupIds") {
        vcr = security_group_ids
            .into_iter()
            .fold(vcr, |vcr, id| vcr.security_group_ids(id));
    }

    if let Some(mut endpoint_public_access) = items.remove("endpointPublicAccess") {
        if let Some(boolean) = endpoint_public_access.pop() {
            let input = boolean
                .parse()
                .map_err(|_| InvalidVpcConfigRequest::invalid_boolean(boolean))?;
            vcr = vcr.endpoint_public_access(input);
        }
    }

    if let Some(mut endpoint_private_access) = items.remove("endpointPrivateAccess") {
        if let Some(boolean) = endpoint_private_access.pop() {
            let input = boolean
                .parse()
                .map_err(|_| InvalidVpcConfigRequest::invalid_boolean(boolean))?;
            vcr = vcr.endpoint_private_access(input);
        }
    }

    if let Some(public_access_cidrs) = items.remove("publicAccessCidrs") {
        vcr = public_access_cidrs
            .into_iter()
            .fold(vcr, |vcr, cidr| vcr.public_access_cidrs(cidr));
    }

    // If there anything else left in the `items` - it is error
    if let Some(item) = items.into_keys().next() {
        Err(InvalidVpcConfigRequest::unknown(item))?
    }

    Ok(vcr.build())
}

#[derive(Clone, Debug, PartialEq, thiserror::Error)]
pub enum InvalidVpcConfigRequest {
    #[error("Unknown Attribute: '{0}'")]
    UnknownAttribute(String),

    #[error("Invalid Boolean: '{0}'")]
    InvalidBoolean(String),
}

impl InvalidVpcConfigRequest {
    fn unknown(text: &str) -> Self {
        Self::UnknownAttribute(text.to_string())
    }

    fn invalid_boolean(boolean: &str) -> Self {
        Self::InvalidBoolean(boolean.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID: &str = "subnetIds=string,string,securityGroupIds=string,string,endpointPublicAccess=true,endpointPrivateAccess=true,publicAccessCidrs=string,string";
    const INVALID: &str = "subnetIds=string,string,securityGroupIds=string,string,endpointPublicAccess=boolean,endpointPrivateAccess=boolean,publicAccessCidrs=string,string";
    const UNKNOWN: &str = "foo=bar,string";

    #[test]
    fn valid() {
        vpc_config_request(VALID).unwrap();
    }

    #[test]
    fn invalid() {
        let err = vpc_config_request(INVALID).unwrap_err();
        assert_eq!(err, InvalidVpcConfigRequest::invalid_boolean("boolean"));
    }

    #[test]
    fn unknown() {
        let err = vpc_config_request(UNKNOWN).unwrap_err();
        assert_eq!(err, InvalidVpcConfigRequest::unknown("foo"));
    }
}
