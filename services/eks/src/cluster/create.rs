use aws_sdk_eks::types::VpcConfigRequest;

use super::*;

/// Creates an Amazon EKS control plane.
#[derive(Debug, Args)]
pub struct CreateCluster {
    /// The unique name to give to your cluster.
    #[arg(long)]
    name: String,

    /// The Amazon Resource Name (ARN) of the IAM role that provides permissions
    /// for the Kubernetes control plane to make calls to Amazon Web Services
    /// API operations on your behalf.
    #[arg(long)]
    role_arn: String,

    /// The VPC configuration that's used by the cluster control plane.
    #[arg(long, value_parser = parsers::eks::vpc_config_request)]
    resources_vpc_config: VpcConfigRequest,
}

impl CreateCluster {
    pub(crate) async fn execute(self, config: &Config) -> EksResult {
        let cluster = config
            .eks()
            .create_cluster()
            .name(self.name)
            .role_arn(self.role_arn)
            .resources_vpc_config(self.resources_vpc_config)
            .send()
            .await?
            .cluster;
        Ok(Box::new(cluster))
    }
}
