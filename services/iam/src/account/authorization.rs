use itertools::Itertools;

use super::*;

/// Retrieves information about all IAM users, groups, roles, and policies in your
/// Amazon Web Services account, including their relationships to one another.
#[derive(Debug, Args)]
pub struct GetAccountAuthorizationDetails {
    /// A list of entity types used to filter the results.
    #[arg(long)]
    filter: Option<Vec<iam::types::EntityType>>,
}

#[async_trait]
impl Execute for GetAccountAuthorizationDetails {
    async fn execute(self: Box<Self>, config: &Config) -> IamResult {
        let items = config
            .client()
            .get_account_authorization_details()
            .set_filter(self.filter)
            .into_paginator()
            .send()
            .try_collect()
            .await?
            .into_iter()
            .map(|output| {
                (
                    output.user_detail_list.unwrap_or_default(),
                    output.group_detail_list.unwrap_or_default(),
                    output.role_detail_list.unwrap_or_default(),
                    output.policies.unwrap_or_default(),
                )
            });

        let (user, group, role, policies): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) =
            itertools::multiunzip(items);

        let user = user.into_iter().flatten().collect_vec();
        let _group = group.into_iter().flatten().collect_vec();
        let _role = role.into_iter().flatten().collect_vec();
        let _policies = policies.into_iter().flatten().collect_vec();

        // Ok(Box::new(user, group, role, policies))
        Ok(Box::new(user))
    }
}
