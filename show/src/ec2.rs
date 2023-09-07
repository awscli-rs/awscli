use aws_sdk_ec2 as ec2;

use super::*;

impl Show for ec2::types::Vpc {
    fn _fmt(&self) -> Box<dyn fmt::Display> {
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

    fn text(&self) -> String {
        fmtools::format!(
            "VPCS\t" { self.cidr_block().text() } "\t"
            { self.dhcp_options_id().text() } "\t"
            { self.instance_tenancy().text() } "\t"
            { self.is_default().text() } "\t"
            { self.owner_id().text() } "\t"
            { self.state().text() } "\t"
            { self.vpc_id().text() } "\t\n"
            { self.ipv6_cidr_block_association_set().text() }
            { self.cidr_block_association_set().text() }
        )

        // VPCS	172.31.0.0/16	dopt-0231e8f83a4087151	default	True	143766252400	available	vpc-0dc4d0edfa9577e0e
        // CIDRBLOCKASSOCIATIONSET	vpc-cidr-assoc-0cba32e6145fb987c	172.31.0.0/16
        // CIDRBLOCKSTATE	associated
    }
}

impl Show for ec2::types::VpcState {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        self.as_str().to_string()
    }
}

impl Show for ec2::types::Tenancy {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        self.as_str().to_string()
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

    fn text(&self) -> String {
        fmtools::format!(
            "IPV6CIDRBLOCKASSOCIATIONSET\t"
            { self.association_id().text() } "\t"
            { self.ipv6_cidr_block().text() } "\t"
            { self.ipv6_cidr_block_state().text() } "\t"
            { self.network_border_group().text() } "\t"
            { self.ipv6_pool().text() }
        )
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

    fn text(&self) -> String {
        fmtools::format!(
            "CIDRBLOCKASSOCIATIONSET\t"
            { self.association_id().text() } "\t"
            { self.cidr_block().text() } "\t\n"
            { self.cidr_block_state().text() }
        )
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

    fn text(&self) -> String {
        fmtools::format!(
            "CIDRBLOCKSTATE\t"
            { self.state().text() } "\t"
            { self.status_message().text() } "\t"
        )
    }
}

impl Show for ec2::types::VpcCidrBlockStateCode {
    fn _fmt(&self) -> Box<dyn fmt::Display + '_> {
        Box::new(self.as_str())
    }

    fn text(&self) -> String {
        self.as_str().to_string()
    }
}
