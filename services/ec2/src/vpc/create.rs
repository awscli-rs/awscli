use super::*;

/// Creates a VPC with the specified CIDR blocks.
#[derive(Debug, Args)]
pub struct CreateVpc {
    /// The IPv4 network range for the VPC, in CIDR notation.
    #[arg(long)]
    cidr_block: Option<String>,

    /// Requests an Amazon-provided IPv6 CIDR block with a /56 prefix length
    /// for the VPC.
    #[arg(long)]
    amazon_provided_ipv6_cidr_block: Option<bool>,

    /// The ID of an IPv6 address pool from which to allocate the IPv6 CIDR block.
    #[arg(long)]
    ipv6_pool: Option<String>,

    /// The IPv6 CIDR block from the IPv6 address pool. You must also
    /// specify Ipv6Pool in the request.
    #[arg(long)]
    ipv6_cidr_block: Option<String>,

    /// The ID of an IPv4 IPAM pool you want to use for allocating this
    /// VPC's CIDR.
    #[arg(long)]
    ipv4_ipam_pool_id: Option<String>,

    /// The netmask length of the IPv4 CIDR you want to allocate to this VPC
    /// from an Amazon VPC IP Address Manager (IPAM) pool.
    #[arg(long)]
    ipv4_netmask_length: Option<i32>,

    /// The ID of an IPv6 IPAM pool which will be used to allocate this VPC
    /// an IPv6 CIDR.
    #[arg(long)]
    ipv6_ipam_pool_id: Option<String>,

    /// The netmask length of the IPv6 CIDR you want to allocate to this VPC
    /// from an Amazon VPC IP Address Manager (IPAM) pool.
    #[arg(long)]
    ipv6_netmask_length: Option<i32>,

    /// Checks whether you have the required permissions for the action,
    /// without actually making the request, and provides an error response.
    #[arg(long)]
    dry_run: Option<bool>,

    /// The tenancy options for instances launched into the VPC.
    #[arg(long,
        // value_parser = ["default", "dedicated", "host"]
        value_parser = clap::value_parser!(types::Tenancy),
    )]
    instance_tenancy: Option<types::Tenancy>,

    /// The name of the location from which we advertise the IPV6 CIDR block.
    #[arg(long)]
    ipv6_cidr_block_network_border_group: Option<String>,

    /// The tags to assign to the VPC.
    #[arg(long, value_parser = parsers::ec2::vpc::parse_tags, num_args = 1..)]
    tag_specifications: Option<Vec<types::TagSpecification>>,

    /// Reads arguments from the JSON string provided.
    #[arg(long)]
    cli_input_json: Option<String>,
}

impl CreateVpc {
    pub(crate) async fn execute(self, config: &Config) -> Ec2Result {
        let vpc = config
            .ec2()
            .create_vpc()
            .set_cidr_block(self.cidr_block)
            .set_amazon_provided_ipv6_cidr_block(self.amazon_provided_ipv6_cidr_block)
            .set_ipv6_pool(self.ipv6_pool)
            .set_ipv6_cidr_block(self.ipv6_cidr_block)
            .set_ipv4_ipam_pool_id(self.ipv4_ipam_pool_id)
            .set_ipv4_netmask_length(self.ipv4_netmask_length)
            .set_ipv6_ipam_pool_id(self.ipv6_ipam_pool_id)
            .set_ipv6_netmask_length(self.ipv6_netmask_length)
            .set_dry_run(self.dry_run)
            .set_instance_tenancy(self.instance_tenancy)
            .set_tag_specifications(self.tag_specifications)
            .send()
            .await?
            .vpc;

        Ok(Box::new(vpc))
    }
}
