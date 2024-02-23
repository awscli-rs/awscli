use aws_sdk_ec2 as ec2;

use super::*;

impl Show for ec2::types::Vpc {
    fn _fmt(&self) -> Box<dyn fmt::Display> {
        // VPCS	172.31.0.0/16	dopt-0231e8f83a4087151	default	True	143766252400	available	vpc-0dc4d0edfa9577e0e
        // CIDRBLOCKASSOCIATIONSET	vpc-cidr-assoc-0cba32e6145fb987c	172.31.0.0/16
        // CIDRBLOCKSTATE	associated

        Box::new(fmtools::format!(
            "VPCS\t" { self.cidr_block()._fmt() } "\t"
            { self.dhcp_options_id()._fmt() } "\t"
            { self.instance_tenancy()._fmt() } "\t"
            { self.is_default()._fmt() } "\t"
            { self.owner_id()._fmt() } "\t"
            { self.state()._fmt() } "\t"
            { self.vpc_id()._fmt() } "\t\n"
            { self.ipv6_cidr_block_association_set()._fmt() }
            { self.cidr_block_association_set()._fmt() }
        ))
    }
}

impl Show for ec2::types::VpcState {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for ec2::types::Tenancy {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}

impl Show for ec2::types::VpcIpv6CidrBlockAssociation {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "IPV6CIDRBLOCKASSOCIATIONSET\t"
            { self.association_id()._fmt() } "\t"
            { self.ipv6_cidr_block()._fmt() } "\t"
            { self.ipv6_cidr_block_state()._fmt() } "\t"
            { self.network_border_group()._fmt() } "\t"
            { self.ipv6_pool()._fmt() }
        ))
    }
}

impl Show for ec2::types::VpcCidrBlockAssociation {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "CIDRBLOCKASSOCIATIONSET\t"
            { self.association_id()._fmt() } "\t"
            { self.cidr_block()._fmt() } "\t\n"
            { self.cidr_block_state()._fmt() }
        ))
    }
}

impl Show for ec2::types::VpcCidrBlockState {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(fmtools::fmt!(
            "CIDRBLOCKSTATE\t"
            { self.state()._fmt() } "\t"
            { self.status_message()._fmt() } "\t"

        ))
    }
}

impl Show for ec2::types::VpcCidrBlockStateCode {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self)
    }
}
